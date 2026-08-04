import { useState } from "react";
import { apiUrl } from "../../config";
import { toast } from "../Toast";

interface Props {
  courseId: string;
  onCancel: () => void;
}

type Difficulty = "Easy" | "Medium" | "Hard";

export default function ProjectIdeaForm({ courseId, onCancel }: Props) {
  const [title, setTitle] = useState("");
  const [body, setBody] = useState("");
  const [difficulty, setDifficulty] = useState<Difficulty>("Medium");
  const [comment, setComment] = useState("");
  const [submitted, setSubmitted] = useState(false);
  const [instantContribute, setInstantContribute] = useState(false);
  // TODO: replace with actual auth check
  const isAdmin = true;

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();

    const endpoint = instantContribute ? "project_idea" : "contributions";
    const requestBody = instantContribute ? {
          Create: {
            course_id: courseId,
            title,
            body,
            difficulty,
          },
        } : {
          Propose: {
            course_id: courseId,
            contribution: {
              Text: {
                AddProjectIdea: {
                  title,
                  body,
                  difficulty,
                },
              },
            },
            comment,
          },
        };

    try {
      const response = await fetch(apiUrl(endpoint), {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        credentials: "include",
        body: JSON.stringify(requestBody),
      });


      if (response.ok) {
        setSubmitted(true);
        toast(instantContribute ? "Project idea published" : "Project idea submitted for review");
      } else {
        const error = await response.text();
        console.error(error);
        toast("Something went wrong", false);
      }

    } catch (error) {
      console.error(error);
      toast("Could not connect to server", false);

    }
  }


  if (submitted) {
    return (
      <div className="text-center py-6 bg-[#F4F2EB] rounded-lg">
        <h3 className="text-xl font-semibold text-[#1A1F3A] mb-2">Thank you for your contribution!</h3>
        <p className="text-[#6B6B5A] mb-6">{instantContribute ? "The project idea has been published." : "Your project idea has been submitted and is waiting for review."}</p>
        <button onClick={onCancel} className="px-4 py-2 bg-[#1A1F3A] text-lg text-white rounded-lg hover:opacity-90">
          Back to course
        </button>
      </div>
    );
  }

  return (
    <form onSubmit={handleSubmit} className="space-y-5">
      <div>
        <label className="block text-lg mb-2 text-[#6B6B5A]">Project title</label>
        <input type="text" placeholder="Example: Build a Rust web server" value={title}
          onChange={(e) => setTitle(e.target.value)} required
          className="w-full border border-[#6B6B5A] rounded-lg px-4 py-3 focus:outline-none"/>
      </div>
      <div>
        <label className="block text-lg mb-2 text-[#6B6B5A]">Description</label>
        <textarea placeholder="Describe the project idea..." value={body}
          onChange={(e) => setBody(e.target.value)} required rows={4}
          className="w-full border border-[#6B6B5A] rounded-lg px-4 py-3 focus:outline-none resize-none"/>
      </div>
      <div>
        <label className="block text-lg mb-2 text-[#6B6B5A]">Difficulty</label>
        <select value={difficulty} onChange={(e) => setDifficulty(e.target.value as Difficulty)}
          className="w-full border border-[#6B6B5A] rounded-lg px-4 py-3 focus:outline-none">
          <option value="Easy">Easy</option>
          <option value="Medium">Medium</option>
          <option value="Hard">Hard</option>
        </select>
      </div>
      <div>
        <label className="block text-lg mb-2 text-[#6B6B5A]">Comment</label>
        <textarea value={comment} placeholder="Anything we should know?"
        onChange={(e) => setComment(e.target.value)} required rows={3}
        className="w-full border border-[#6B6B5A] rounded-lg px-4 py-3 resize-none focus:outline-none"/>
      </div>
      <div className="flex items-center justify-between pt-2">
        <div>
          {isAdmin && (
            <label className="flex items-center gap-2 text-[#6B6B5A]">
              <input type="checkbox" checked={instantContribute}
              onChange={(e) => setInstantContribute(e.target.checked)}/>
              Instant contribute</label>
          )}
        </div>
        <div className="flex gap-3">
          <button onClick={onCancel}className="px-4 py-2 text-lg text-[#6B6B5A] border border-[#6B6B5A] rounded-lg hover:bg-gray-100">
            Cancel
          </button>
          <button type="submit" className="px-4 py-2 text-lg bg-[#1A1F3A] text-white rounded-lg hover:opacity-90 disabled:opacity-50">
            {instantContribute ? "Publish" : "Send for review"}
          </button>
        </div>
      </div>
    </form>
  );
}