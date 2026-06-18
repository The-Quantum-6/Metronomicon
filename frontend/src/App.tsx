import { Routes, Route } from "react-router-dom";
import Navbar from "./modules/navBar";
import Home from "./modules/Home";
import About from "./modules/About";

function App() {
  return (
    <div>
      <Navbar />
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/about" element={<About />} />
      </Routes>
    </div>
  );
}

export default App;
