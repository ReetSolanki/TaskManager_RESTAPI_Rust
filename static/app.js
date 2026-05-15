const API = '/tasks';

async function loadTasks() {
    const res = await fetch(API);
    const tasks = await res.json();
    const list = document.getElementById('taskList');

    if (tasks.length === 0) {
    list.innerHTML = '<p class="empty">No tasks yet. Add one above.</p>';
    return;
    }

    list.innerHTML = tasks.map(t => renderTask(t)).join('');
}

function renderTask(t) {
    return `
    <div class="task" id="task-${t.id}">
        <input type="checkbox" ${t.done ? 'checked' : ''}
        onchange="toggleTask(${t.id}, '${escapeQuotes(t.title)}', this.checked)" />
        <span class="task-title ${t.done ? 'done' : ''}">${t.title}</span>
        <button class="btn-edit" onclick="startEdit(${t.id}, '${escapeQuotes(t.title)}', ${t.done})">✏️</button>
        <button class="btn-delete" onclick="deleteTask(${t.id})">✕</button>
    </div>
    `;
}

function renderEditMode(t_id, title, done) {
    return `
    <div class="task" id="task-${t_id}">
        <input type="checkbox" ${done ? 'checked' : ''}
        onchange="toggleTask(${t_id}, '${escapeQuotes(title)}', this.checked)" />
        <input class="edit-input" id="edit-input-${t_id}" value="${title}" />
        <button class="btn-save"   onclick="saveEdit(${t_id}, ${done})">Save</button>
        <button class="btn-cancel" onclick="loadTasks()">Cancel</button>
    </div>
    `;
}

function startEdit(id, title, done) {
    const taskEl = document.getElementById(`task-${id}`);
    taskEl.outerHTML = renderEditMode(id, title, done);
    document.getElementById(`edit-input-${id}`).focus();
}

async function saveEdit(id, done) {
    const input = document.getElementById(`edit-input-${id}`);
    const newTitle = input.value.trim();
    if (!newTitle) return;

    await fetch(`${API}/${id}`, {
    method: 'PUT',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ title: newTitle, done })
    });

    loadTasks();
}

async function addTask() {
    const input = document.getElementById('taskInput');
    const title = input.value.trim();
    if (!title) return;

    await fetch(API, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ title })
    });

    input.value = '';
    loadTasks();
}

async function toggleTask(id, title, done) {
    await fetch(`${API}/${id}`, {
    method: 'PUT',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ title, done })
    });
    loadTasks();
}

async function deleteTask(id) {
    await fetch(`${API}/${id}`, { method: 'DELETE' });
    loadTasks();
}

function escapeQuotes(str) {
    return str.replace(/'/g, "\\'");
}

document.addEventListener('DOMContentLoaded', () => {
    document.getElementById('taskInput')
    .addEventListener('keydown', e => {
        if (e.key === 'Enter') addTask();
    });
    loadTasks();
});