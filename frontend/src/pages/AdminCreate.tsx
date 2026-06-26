import Navbar from "../components/Navbar";
import { useState, type FormEvent } from "react";
import { useNavigate } from "react-router-dom";
import { apiUrl } from "../config";
import ReactQuill from "react-quill-new";
import "react-quill-new/dist/quill.snow.css";

export default function AdminCreate() {
  const [name, setName] = useState("");
  const [code, setCode] = useState("");
  const [content, setContent] = useState("");
  const navigate = useNavigate();

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    const payload = { name, content, code };
    const res = await fetch(apiUrl("courses/create"), {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(payload),
    });
    if (res.ok) navigate("/admin");
    else alert("Create failed");
  };

  // Quill toolbar/modules to support headers, lists and common formats
  const modules = {
    toolbar: [
      [{ header: [1, 2, 3, false] }],
      ["bold", "italic", "underline", "strike"],
      [{ list: "ordered" }, { list: "bullet" }],
      ["link", "blockquote", "code-block"],
      ["clean"],
    ],
  };

  const formats = [
    "header",
    "bold",
    "italic",
    "underline",
    "strike",
    "list",
    "link",
    "blockquote",
    "code-block",
  ];

  return (
    <div className="min-h-screen bg-surface-dark text-text">
      <Navbar />
      <main className="container mx-auto p-6">
        <h1 className="text-2xl font-bold mb-6 text-primary">Create Course</h1>
        <form onSubmit={handleSubmit} className="space-y-4 bg-bg p-6 rounded border border-surface">
          <div>
            <label className="block text-sm font-medium text-text-secondary">Name</label>
            <input className="mt-1 block w-full bg-surface-dark border border-surface rounded p-2 text-text focus:border-accent focus:outline-none" value={name} onChange={(e) => setName(e.target.value)} required />
          </div>
          <div>
            <label className="block text-sm font-medium text-text-secondary">Code</label>
            <input className="mt-1 block w-full bg-surface-dark border border-surface rounded p-2 text-text focus:border-accent focus:outline-none" value={code} onChange={(e) => setCode(e.target.value)} required />
          </div>

          <div>
            <label className="block text-sm font-medium text-text-secondary mb-2">Content (rich text)</label>
            <ReactQuill theme="snow" value={content} onChange={setContent} modules={modules} formats={formats} />
            <p className="text-sm text-placeholder mt-2">Preview (raw HTML stored):</p>
            <pre className="mt-2 p-3 bg-surface rounded max-h-40 overflow-auto text-xs text-text-secondary">{content}</pre>
          </div>

          <div>
            <button type="submit" className="bg-accent hover:bg-accent-dark text-white px-4 py-2 rounded transition">Create</button>
          </div>
        </form>
      </main>
    </div>
  );
}
