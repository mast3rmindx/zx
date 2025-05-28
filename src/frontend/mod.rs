use yew::prelude::*;
use web_sys::HtmlCanvasElement;
use wasm_bindgen::JsCast;
use gloo::timers::callback::Interval;
use plotters::prelude::*;
use plotters_backend::DrawingBackend;
use crate::{Node, Edge};

/// Main component for DAG visualization
#[function_component(DagVisualization)]
pub fn dag_visualization() -> Html {
    let nodes = use_state(Vec::new);
    let edges = use_state(Vec::new);
    let canvas_ref = use_node_ref();

    // Fetch data periodically
    {
        let nodes = nodes.clone();
        let edges = edges.clone();
        use_effect_with_deps(
            move |_| {
                let interval = Interval::new(1000, move || {
                    // Fetch nodes
                    wasm_bindgen_futures::spawn_local(async move {
                        let response = reqwest::get("/api/nodes").await.unwrap();
                        let new_nodes: Vec<Node> = response.json().await.unwrap();
                        nodes.set(new_nodes);
                    });

                    // Fetch edges
                    wasm_bindgen_futures::spawn_local(async move {
                        let response = reqwest::get("/api/edges").await.unwrap();
                        let new_edges: Vec<Edge> = response.json().await.unwrap();
                        edges.set(new_edges);
                    });
                });
                || drop(interval)
            },
            (),
        );
    }

    // Draw the graph when data changes
    {
        let canvas_ref = canvas_ref.clone();
        let nodes = nodes.clone();
        let edges = edges.clone();
        use_effect_with_deps(
            move |(nodes, edges)| {
                if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                    draw_graph(&canvas, nodes, edges);
                }
                || ()
            },
            ((*nodes).clone(), (*edges).clone()),
        );
    }

    html! {
        <div class="dag-visualization">
            <canvas ref={canvas_ref} width="800" height="600" />
        </div>
    }
}

/// Draw the graph using plotters
fn draw_graph(canvas: &HtmlCanvasElement, nodes: &[Node], edges: &[Edge]) {
    let backend = CanvasBackend::with_canvas_object(canvas).unwrap();
    let root = backend.into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .build_cartesian_2d(-1f32..1f32, -1f32..1f32)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    // Calculate node positions using a simple circular layout
    let node_positions: HashMap<&str, (f32, f32)> = nodes
        .iter()
        .enumerate()
        .map(|(i, node)| {
            let angle = 2.0 * std::f32::consts::PI * (i as f32) / (nodes.len() as f32);
            let x = 0.8 * angle.cos();
            let y = 0.8 * angle.sin();
            (node.id.as_str(), (x, y))
        })
        .collect();

    // Draw edges
    for edge in edges {
        if let (Some(&(x1, y1)), Some(&(x2, y2))) = (
            node_positions.get(edge.from.as_str()),
            node_positions.get(edge.to.as_str()),
        ) {
            chart
                .draw_series(std::iter::once(PathElement::new(
                    vec![(x1, y1), (x2, y2)],
                    &BLACK.mix(0.5),
                )))
                .unwrap();

            // Draw arrow
            let dx = x2 - x1;
            let dy = y2 - y1;
            let len = (dx * dx + dy * dy).sqrt();
            let norm_dx = dx / len;
            let norm_dy = dy / len;
            let arrow_size = 0.05;

            let arrow_x = x2 - arrow_size * norm_dx;
            let arrow_y = y2 - arrow_size * norm_dy;
            let arrow_dx = arrow_size * norm_dy;
            let arrow_dy = -arrow_size * norm_dx;

            chart
                .draw_series(std::iter::once(PathElement::new(
                    vec![
                        (arrow_x + arrow_dx, arrow_y + arrow_dy),
                        (x2, y2),
                        (arrow_x - arrow_dx, arrow_y - arrow_dy),
                    ],
                    &BLACK,
                )))
                .unwrap();
        }
    }

    // Draw nodes
    for (node_id, &(x, y)) in &node_positions {
        chart
            .draw_series(std::iter::once(Circle::new(
                (x, y),
                5,
                &BLUE.mix(0.5).filled(),
            )))
            .unwrap();

        chart
            .draw_series(std::iter::once(Text::new(
                node_id.to_string(),
                (x, y + 0.1),
                ("sans-serif", 15).into_font(),
            )))
            .unwrap();
    }
} 