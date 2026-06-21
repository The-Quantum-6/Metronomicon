function DeleteButton({ id, onDelete }: { id: string; onDelete: () => void }) {
  async function handleDelete() {
    const res = await fetch(`/admin/course/${id}`, { method: 'DELETE' });
    if (res.ok) onDelete();
  }
  return <button onClick={handleDelete}>🗑</button>;
}

export default DeleteButton;
