import { Routes, Route } from "react-router-dom";
import Home from "./pages/Home";
import Login from "./pages/Login";
import "@navikt/ds-react";
import "@navikt/ds-css";
import CoursePage from "./pages/Course";
import Admin from "./pages/Admin";
import AdminEdit from "./pages/AdminEdit";
import Profile from "./pages/Profile";
import StaffPortal from "./pages/StaffPortal";
import About from "./pages/About";

function App() {
  return (
    <Routes>
      <Route path="/" element={<Home />} />
      
      <Route path="/login" element={<Login />} />
      <Route path="/about" element={<About />} />
      <Route path="/profile" element={<Profile />} />
      <Route path="/staff" element={<StaffPortal />} />
      <Route path="/courses/:id" element={<CoursePage />} />
      <Route path="/admin" element={<Admin />} />
      <Route path="/admin/edit/:id" element={<AdminEdit />} />
    </Routes>
  );
}

export default App;
