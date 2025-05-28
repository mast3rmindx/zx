import React from 'react';
import './App.css';
import DagVisualization from './components/DagVisualization';

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <h1>KnightDAG Visualization</h1>
      </header>
      <main>
        <DagVisualization />
      </main>
    </div>
  );
}

export default App;
