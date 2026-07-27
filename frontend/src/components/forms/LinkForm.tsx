import { useState } from "react";
import { apiUrl } from "../../config";
import type { ContributeMode } from "../Contribute";

interface Props {
  courseId: string;
  mode: ContributeMode;
  onCancel: () => void;
}

export default function LinkForm({ courseId, mode, onCancel }: Props) {
  const [label, setLabel] = useState("");
  const [url, setUrl] = useState("");
  const [submitted, setSubmitted] = useState(false);

  async function handleSubmit(e: React.SubmitEvent) {
    e.preventDefault();

    const endpoint = mode === "direct" ? "links" : "contributions";
    const payload =
      mode === "direct"
        ? { Create: { course_id: courseId, label, url } }
        : {
            Propose: {
              course_id: courseId,
              contribution: { Text: { AddLink: { label, url } } },
              comment: "",
            },
          };

    try {
      const response = await fetch(apiUrl(endpoint), {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        credentials: "include",
        body: JSON.stringify(payload),
      });

      if (response.ok) {
        setSubmitted(true);
      } else {
        //// TODO: Replace this with a better error handling
        const error = await response.text();
        console.error(error);
        alert("Something went wrong.");
      }
    } 
    catch (error) {
      console.error(error);
      alert("Could not connect to server.");
    }
  }

  if (submitted) {
    return (
      <div className="text-center py-6 bg-[#F4F2EB] rounded-lg">
        <h3 className="text-xl font-semibold text-[#1A1F3A] mb-2">Thank you for your contribution!</h3>
        <p className="text-[#6B6B5A] mb-6">
          {mode === "direct"
            ? "Your link has been published."
            : "Your link has been submitted and is waiting for review."}
        </p>
        <button onClick={onCancel} className="px-4 py-2 bg-[#1A1F3A] text-lg text-white rounded-lg hover:opacity-90 transition-colors">Back to course
        </button>
      </div>
    );
  }

  return (
    <form onSubmit={handleSubmit} className="space-y-5">
      <div>
        <label className="block text-lg mb-2 text-[#6B6B5A]">Label</label>
        <input type="text" placeholder="Example: Understand Git better..." value={label} 
        onChange={(e) => setLabel(e.target.value)} required
        className="w-full border border-[#6B6B5A] rounded-lg px-4 py-3 focus:outline-none"/>
      </div>
      <div>
        <label className="block text-lg mb-2 text-[#6B6B5A]">URL</label>
        <input type="url" placeholder="https://..." value={url}
        onChange={(e) => setUrl(e.target.value)}
        required
        className="w-full border border-[#6B6B5A] rounded-lg px-4 py-3 focus:outline-none"/>
      </div>
      <div className="flex justify-end gap-3 pt-2ß">
        <button type="button" onClick={onCancel} className="px-4 py-2 text-lg text-[#6B6B5A] border border-[#6B6B5A] rounded-lg hover:bg-gray-100 transition-colors">Cancel
        </button>
        <button type="submit" className="px-4 py-2 text-lg bg-[#02061b] text-white rounded-lg hover:opacity-90 transition-colors disabled:opacity-50">{mode === "direct" ? "Publish" : "Send for review"}
        </button>
      </div>
    </form>
  );
}