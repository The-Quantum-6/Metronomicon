import { Routes, Route } from "react-router-dom";
import Home from "./pages/Home";
import Login from "./pages/Login";
import "@navikt/ds-react";
import "@navikt/ds-css";
import CoursePage from "./pages/Course";
import Admin from "./pages/Admin";
import AdminCreate from "./pages/AdminCreate";
import AdminEdit from "./pages/AdminEdit";

function App() {
  return (
    <Routes>
      <Route path="/" element={<Home />} />
      
      <Route path="/login" element={<Login />} />
      <Route path="/courses/:id" element={<CoursePage />} />
      <Route path="/admin" element={<Admin />} />
      <Route path="/admin/create" element={<AdminCreate />} />
      <Route path="/admin/edit/:id" element={<AdminEdit />} />
    </Routes>
  );
}

export default App;
