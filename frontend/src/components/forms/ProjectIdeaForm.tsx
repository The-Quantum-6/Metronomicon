import { useState } from "react";
import { apiUrl } from "../../config";

interface Props {
  courseId: string;
  onCancel: () => void;
}

type Difficulty = "Easy" | "Medium" | "Hard";

export default function ProjectIdeaForm({ courseId, onCancel }: Props) {
  const [title, setTitle] = useState("");
  const [body, setBody] = useState("");
  const [difficulty, setDifficulty] = useState<Difficulty>("Medium");
  const [submitted, setSubmitted] = useState(false);


  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();


    try {
      const response = await fetch(apiUrl("project_idea"), {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          Create: {
            course_id: courseId,
            title,
            body,
            difficulty,
          },
        }),
      });


      if (response.ok) {
        setSubmitted(true);
      } else {
        // TODO: Replace this with a better error handling
        const error = await response.text();
        console.error(error);
        alert("Something went wrong.");
      }

    } catch (error) {
      console.error(error);
      alert("Could not connect to server.");

    }
  }


  if (submitted) {
    return (
      <div className="text-center py-6">

        <h3 className="text-xl font-semibold text-[#1A1F3A] mb-2">
          Thank you for your contribution!
        </h3>

        <p className="text-[#6B6B5A] mb-6">
          Your project idea has been submitted and is waiting for review.
        </p>

        <button
          type="button"
          onClick={onCancel}
          className="px-4 py-2 bg-[#1A1F3A] text-white rounded-lg hover:opacity-90"
        >
          Back to course
        </button>

      </div>
    );
  }


  return (
    <form onSubmit={handleSubmit} className="space-y-5">

      <div>
        <label className="block text-sm mb-1 text-[#6B6B5A]">
          Project title
        </label>

        <input
          type="text"
          placeholder="Example: Build a Rust web server"
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          required
          className="w-full border border-[#6B6B5A] rounded-lg px-4 py-2 focus:outline-none"
        />
      </div>


      <div>
        <label className="block text-sm mb-1 text-[#6B6B5A]">
          Description
        </label>

        <textarea
          placeholder="Describe the project idea..."
          value={body}
          onChange={(e) => setBody(e.target.value)}
          required
          rows={5}
          className="w-full border border-[#6B6B5A] rounded-lg px-4 py-3 focus:outline-none resize-none"
        />
      </div>


      <div>
        <label className="block text-sm mb-1 text-[#6B6B5A]">
          Difficulty
        </label>

        <select
          value={difficulty}
          onChange={(e) =>
            setDifficulty(e.target.value as Difficulty)
          }
          className="w-full border border-[#6B6B5A] rounded-lg px-4 py-2 focus:outline-none"
        >
          <option value="Easy">
            Easy
          </option>

          <option value="Medium">
            Medium
          </option>

          <option value="Hard">
            Hard
          </option>

        </select>
      </div>


      <div className="flex justify-end gap-3 pt-4">

        <button
          type="button"
          onClick={onCancel}
          className="px-4 py-2 text-lg text-[#6B6B5A] border border-[#6B6B5A] rounded-lg hover:bg-gray-100 transition-colors"
        >
          Cancel
        </button>


        <button
          type="submit"
          className="px-4 py-2 text-lg bg-[#1A1F3A] text-white rounded-lg hover:opacity-90 disabled:opacity-50 transition-colors"
        >Send for review
          
        </button>

      </div>

    </form>
  );
}