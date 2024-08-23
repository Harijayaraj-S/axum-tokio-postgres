document.addEventListener('DOMContentLoaded', () => {
    const openModalButton = document.getElementById('open-modal');
    const closeModalButton = document.getElementById('close-modal');
    const modal = document.getElementById('task-modal');
    const form = document.getElementById('task-form');
    const taskList = document.getElementById('task-list');

    // Open modal
    openModalButton.addEventListener('click', () => {
        modal.style.display = 'block';
    });

    // Close modal
    closeModalButton.addEventListener('click', () => {
        modal.style.display = 'none';
    });

    // Submit form
    form.addEventListener('submit', async (event) => {
        event.preventDefault();

        const title = document.getElementById('title').value;
        const priority = document.getElementById('priority').value;
        const description = document.getElementById('description').value;

        const response = await fetch('/create', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ title, priority, description }),
        });

        const result = await response.json();
        if (result.success) {
            modal.style.display = 'none'; // Close the modal
            fetchTasks(); // Refresh the task list
        } else {
            console.error('Task creation failed');
        }
    });

    // Fetch and display tasks
    const fetchTasks = async () => {
        const response = await fetch('/list');
        const html = await response.text();
        taskList.innerHTML = html;
    };

    fetchTasks();
});
