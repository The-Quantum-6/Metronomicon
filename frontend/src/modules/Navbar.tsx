import { Link } from "react-router-dom";

function Navbar() {
  return (
    <nav
      aria-label="Primary"
      style={{ display: "flex", gap: "1rem", padding: "1rem", background: "#222" }}
    >
      <Link to="/" style={{ color: "#fff" }}>
        Home
      </Link>
      <Link to="/about" style={{ color: "#fff" }}>
        About
      </Link>
    </nav>
  );
}

export default Navbar;