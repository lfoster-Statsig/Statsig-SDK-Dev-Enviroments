import "./App.css";

import { StatsigProvider, useStatsigClient } from "@statsig/react-bindings";

import logo from "./logo.svg";

function AppContent() {
  const client = useStatsigClient();
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <div>
          Gate is {client.checkGate("new_feature_gate") ? "passing" : "failing"}
          .
        </div>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

function App() {
  return (
    <StatsigProvider
      sdkKey={process.env.REACT_APP_CLIENT_KEY}
      user={{ userID: "loganfoster" }}
      options={{ environment: { tier: "development" } }}
    >
      <AppContent />
    </StatsigProvider>
  );
}

export default App;
