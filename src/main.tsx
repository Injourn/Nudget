import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles.css";
import { BrowserRouter } from "react-router-dom";

// forwardConsole('log', trace);
// forwardConsole('debug', debug);
// forwardConsole('info', info);
// forwardConsole('warn', warn);
// forwardConsole('error', error);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <BrowserRouter>
    <App />
  </BrowserRouter>,
);
