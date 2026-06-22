import { apiUrl } from '../../config';

function DeleteButton({ id, onDelete }: { id: string; onDelete: () => void }) {
  async function handleDelete() {
    const res = await fetch(apiUrl(`courses/${id}`), { method: 'DELETE' });
    if (res.ok) onDelete();
  }
  return <button onClick={handleDelete}>🗑</button>;
}

export default DeleteButton;
