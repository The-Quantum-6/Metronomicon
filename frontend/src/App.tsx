import { Routes, Route } from "react-router-dom";
import Home from "./pages/Home";
import Login from "./pages/Login";
import CoursePage from "./pages/Course";
import Admin from "./pages/Admin";
import AdminCreate from "./pages/AdminCreate";
import AdminEdit from "./pages/AdminEdit";
import Profile from "./pages/Profile";
import StaffPortal from "./pages/StaffPortal";

function App() {
  return (
    <Routes>
      <Route path="/" element={<Home />} />
      
      <Route path="/login" element={<Login />} />
      <Route path="/profile" element={<Profile />} />
      <Route path="/staff" element={<StaffPortal />} />
      <Route path="/courses/:id" element={<CoursePage />} />
      <Route path="/admin" element={<Admin />} />
      <Route path="/admin/create" element={<AdminCreate />} />
      <Route path="/admin/edit/:id" element={<AdminEdit />} />
    </Routes>
  );
}

export default App;
