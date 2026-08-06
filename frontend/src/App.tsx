import { Routes, Route } from "react-router-dom";
import Home from "./pages/Home";
import Login from "./pages/Login";
import "@navikt/ds-react";
import "@navikt/ds-css";
import CoursePage from "./pages/Course";
import Admin from "./pages/Admin";
import AdminEdit from "./pages/AdminEdit";
import StaffPortal from "./pages/StaffPortal";
import PageReports from "./pages/PageReports";
import About from "./pages/About";
import PrivacyPolicy from "./pages/PrivacyPolicy";
import Cookies from "./pages/Cookies";
import CookieBanner from "./components/CookieBanner";

function App() {
  return (
    <Routes>
      <Route path="/" element={<Home />} />
      
      <Route path="/login" element={<Login />} />
      <Route path="/about" element={<About />} />
      <Route path="/staff" element={<StaffPortal />} />
      <Route path="/pageReports" element={<PageReports />} />
      <Route path="/privacy" element={<PrivacyPolicy />} />
      <Route path="/cookies" element={<Cookies />} />
      <Route path="/courses/:id" element={<CoursePage />} />
      <Route path="/admin" element={<Admin />} />
      <Route path="/admin/edit/:id" element={<AdminEdit />} />
      < CookieBanner />
    </Routes>
  );
}

export default App;
