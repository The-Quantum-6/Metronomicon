import { Link } from "react-router-dom";

function Navbar() {
  return (
    <nav className="flex gap-6 p-4 bg-gray-900 border-b-2 border-purple-600" aria-label="Primary">
      <Link to="/" className="text-white hover:text-purple-400 transition">Home</Link>
      <Link to="/login" className="text-white hover:text-purple-400 transition">Login</Link>
    </nav>
  );
}

export default Navbar;