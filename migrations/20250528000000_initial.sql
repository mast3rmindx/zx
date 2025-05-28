-- Create nodes table
CREATE TABLE nodes (
    id TEXT PRIMARY KEY,
    data JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    description TEXT,
    attributes JSONB NOT NULL DEFAULT '{}'::jsonb
);

-- Create edges table
CREATE TABLE edges (
    from_node TEXT NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    to_node TEXT NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    weight DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    description TEXT,
    attributes JSONB NOT NULL DEFAULT '{}'::jsonb,
    PRIMARY KEY (from_node, to_node)
);

-- Create index for faster edge lookups
CREATE INDEX edges_to_node_idx ON edges(to_node);

-- Create function to check for cycles
CREATE OR REPLACE FUNCTION check_dag_cycle(p_from_node TEXT, p_to_node TEXT)
RETURNS BOOLEAN AS $$
DECLARE
    found_cycle BOOLEAN;
BEGIN
    WITH RECURSIVE search_graph(from_node, to_node, path, cycle) AS (
        -- Initial case: direct edge
        SELECT e.from_node, e.to_node, ARRAY[e.from_node], false
        FROM edges e
        WHERE e.from_node = p_to_node
        
        UNION ALL
        
        -- Recursive case: follow edges
        SELECT e.from_node, e.to_node, path || e.from_node, e.to_node = ANY(path)
        FROM edges e
        INNER JOIN search_graph sg ON e.from_node = sg.to_node
        WHERE NOT cycle
    )
    SELECT EXISTS (
        SELECT 1 FROM search_graph WHERE cycle OR to_node = p_from_node
    ) INTO found_cycle;
    
    RETURN found_cycle;
END;
$$ LANGUAGE plpgsql; 