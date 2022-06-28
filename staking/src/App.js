import "./App.css";

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <box style={{ border: "1px solid black", padding:"15px", borderRadius: "10px", display: "flex", flexDirection: "row" }}>
          <input placeholder="Public Address" style={{margin:"0 5px"}} />
          <br />
          <button>search</button>
        </box>
      </header>
    </div>
  );
}

export default App;
