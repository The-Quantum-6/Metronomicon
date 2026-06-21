import { Routes, Route } from "react-router-dom";
import Home from "./pages/Home";
import Login from "./pages/Login";
import CoursePage from "./pages/Course";
import Admin from "./pages/Admin";
import AdminCreate from "./pages/AdminCreate";

function App() {
  return (
    <Routes>
      <Route path="/" element={<Home />} />
      
      <Route path="/login" element={<Login />} />
      <Route path="/courses/:id" element={<CoursePage />} />
      <Route path="/admin" element={<Admin />} />
      <Route path="/admin/create" element={<AdminCreate />} />
    </Routes>
  );
}

export default App;
