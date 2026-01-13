<script lang="ts">
  import { instructions, createInstruction, updateInstruction, deleteInstruction, importInstructionFromText, exportInstructionToMarkdown, selectInstruction, selectedInstruction } from '$lib/stores';
  import type { Instruction, InstructionCategory } from '$lib/types';
  import { defaultInstruction } from '$lib/types';

  let showNewForm = false;
  let showImportModal = false;
  let importText = '';
  let editingInstruction: Instruction | null = null;

  // Form state
  let formData = { ...defaultInstruction } as Partial<Instruction>;

  const categories: { value: InstructionCategory; label: string; emoji: string }[] = [
    { value: 'general', label: 'General', emoji: '📝' },
    { value: 'code_style', label: 'Code Style', emoji: '📏' },
    { value: 'communication', label: 'Communication', emoji: '💬' },
    { value: 'workflow', label: 'Workflow', emoji: '🔄' },
    { value: 'security', label: 'Security', emoji: '🔒' },
    { value: 'testing', label: 'Testing', emoji: '🧪' },
    { value: 'documentation', label: 'Documentation', emoji: '📚' },
    { value: 'custom', label: 'Custom', emoji: '⚙️' }
  ];

  function resetForm() {
    formData = { ...defaultInstruction };
    editingInstruction = null;
    showNewForm = false;
  }

  function startEditing(instruction: Instruction) {
    editingInstruction = instruction;
    formData = { ...instruction };
    showNewForm = true;
  }

  async function handleSubmit() {
    if (editingInstruction) {
      await updateInstruction({ ...editingInstruction, ...formData } as Instruction);
    } else {
      await createInstruction(formData);
    }
    resetForm();
  }

  async function handleDelete(id: string) {
    if (confirm('Are you sure you want to delete this instruction?')) {
      await deleteInstruction(id);
      if ($selectedInstruction?.id === id) {
        selectInstruction(null);
      }
    }
  }

  async function handleImport() {
    if (importText.trim()) {
      const imported = await importInstructionFromText(importText);
      if (imported) {
        showImportModal = false;
        importText = '';
      }
    }
  }

  async function handleExport(id: string) {
    const markdown = await exportInstructionToMarkdown(id);
    if (markdown) {
      await navigator.clipboard.writeText(markdown);
      alert('Instruction copied to clipboard!');
    }
  }

  function getCategoryEmoji(category: InstructionCategory): string {
    return categories.find(c => c.value === category)?.emoji || '📋';
  }
</script>

<div class="instructions-view">
  <header class="view-header">
    <div class="header-left">
      <h1>📋 Instructions</h1>
      <p class="subtitle">Reusable instruction sets for Claude</p>
    </div>
    <div class="header-actions">
      <button class="btn btn-secondary" onclick={() => showImportModal = true}>
        📥 Import
      </button>
      <button class="btn btn-primary" onclick={() => showNewForm = true}>
        ➕ New Instruction
      </button>
    </div>
  </header>

  {#if showImportModal}
    <div class="modal-overlay" onclick={() => showImportModal = false}>
      <div class="modal" onclick={(e) => e.stopPropagation()}>
        <h2>Import Instruction</h2>
        <p>Paste YAML frontmatter or markdown content:</p>
        <textarea
          bind:value={importText}
          placeholder="---
name: My Instruction
category: code_style
priority: 7
---

# Instructions content here..."
          rows="12"
        ></textarea>
        <div class="modal-actions">
          <button class="btn btn-secondary" onclick={() => showImportModal = false}>Cancel</button>
          <button class="btn btn-primary" onclick={handleImport}>Import</button>
        </div>
      </div>
    </div>
  {/if}

  <div class="instructions-content">
    <div class="instructions-list">
      {#each $instructions as instruction}
        <button
          class="instruction-card"
          class:selected={$selectedInstruction?.id === instruction.id}
          onclick={() => selectInstruction(instruction.id)}
        >
          <div class="card-header">
            <span class="emoji">{instruction.icon_emoji}</span>
            <div class="card-info">
              <h3>{instruction.name}</h3>
              <span class="category-badge">{getCategoryEmoji(instruction.category)} {instruction.category}</span>
            </div>
            <div class="card-meta">
              <span class="priority" title="Priority">⚡ {instruction.priority}</span>
              <span class="status" class:enabled={instruction.enabled}>
                {instruction.enabled ? '✓' : '○'}
              </span>
            </div>
          </div>
          <p class="description">{instruction.description || 'No description'}</p>
          {#if instruction.tags.length > 0}
            <div class="tags">
              {#each instruction.tags as tag}
                <span class="tag">{tag}</span>
              {/each}
            </div>
          {/if}
        </button>
      {/each}

      {#if $instructions.length === 0}
        <div class="empty-state">
          <span class="emoji">📋</span>
          <p>No instructions yet</p>
          <button class="btn btn-primary" onclick={() => showNewForm = true}>
            Create your first instruction
          </button>
        </div>
      {/if}
    </div>

    <div class="instruction-detail">
      {#if showNewForm}
        <form class="instruction-form" onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
          <h2>{editingInstruction ? 'Edit Instruction' : 'New Instruction'}</h2>

          <div class="form-row">
            <div class="form-group emoji-picker">
              <label>Icon</label>
              <input type="text" bind:value={formData.icon_emoji} maxlength="2" />
            </div>
            <div class="form-group flex-grow">
              <label>Name</label>
              <input type="text" bind:value={formData.name} placeholder="Instruction name" required />
            </div>
          </div>

          <div class="form-group">
            <label>Description</label>
            <input type="text" bind:value={formData.description} placeholder="Brief description" />
          </div>

          <div class="form-row">
            <div class="form-group">
              <label>Category</label>
              <select bind:value={formData.category}>
                {#each categories as cat}
                  <option value={cat.value}>{cat.emoji} {cat.label}</option>
                {/each}
              </select>
            </div>
            <div class="form-group">
              <label>Priority (1-10)</label>
              <input type="number" bind:value={formData.priority} min="1" max="10" />
            </div>
            <div class="form-group checkbox-group">
              <label>
                <input type="checkbox" bind:checked={formData.enabled} />
                Enabled
              </label>
            </div>
          </div>

          <div class="form-group">
            <label>Content (Markdown)</label>
            <textarea
              bind:value={formData.content}
              placeholder="# Instructions

Write your instructions here using Markdown..."
              rows="12"
            ></textarea>
          </div>

          <div class="form-group">
            <label>Tags (comma-separated)</label>
            <input
              type="text"
              value={formData.tags?.join(', ') || ''}
              oninput={(e) => formData.tags = (e.target as HTMLInputElement).value.split(',').map(t => t.trim()).filter(Boolean)}
              placeholder="code, style, best-practices"
            />
          </div>

          <div class="form-actions">
            <button type="button" class="btn btn-secondary" onclick={resetForm}>Cancel</button>
            <button type="submit" class="btn btn-primary">
              {editingInstruction ? 'Update' : 'Create'} Instruction
            </button>
          </div>
        </form>
      {:else if $selectedInstruction}
        <div class="instruction-preview">
          <div class="preview-header">
            <span class="emoji large">{$selectedInstruction.icon_emoji}</span>
            <div class="preview-info">
              <h2>{$selectedInstruction.name}</h2>
              <p>{$selectedInstruction.description}</p>
            </div>
          </div>

          <div class="preview-meta">
            <span class="meta-item">
              {getCategoryEmoji($selectedInstruction.category)} {$selectedInstruction.category}
            </span>
            <span class="meta-item">⚡ Priority: {$selectedInstruction.priority}</span>
            <span class="meta-item status" class:enabled={$selectedInstruction.enabled}>
              {$selectedInstruction.enabled ? '✓ Enabled' : '○ Disabled'}
            </span>
          </div>

          <div class="preview-content">
            <h3>Content</h3>
            <pre>{$selectedInstruction.content}</pre>
          </div>

          {#if $selectedInstruction.tags.length > 0}
            <div class="preview-tags">
              <h3>Tags</h3>
              <div class="tags">
                {#each $selectedInstruction.tags as tag}
                  <span class="tag">{tag}</span>
                {/each}
              </div>
            </div>
          {/if}

          <div class="preview-actions">
            <button class="btn btn-secondary" onclick={() => handleExport($selectedInstruction!.id)}>
              📤 Export
            </button>
            <button class="btn btn-secondary" onclick={() => startEditing($selectedInstruction!)}>
              ✏️ Edit
            </button>
            <button class="btn btn-danger" onclick={() => handleDelete($selectedInstruction!.id)}>
              🗑️ Delete
            </button>
          </div>
        </div>
      {:else}
        <div class="empty-detail">
          <span class="emoji">👈</span>
          <p>Select an instruction to view details</p>
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .instructions-view {
    display: flex;
    flex-direction: column;
    height: 100%;
    animation: fade-in 0.5s ease-out;
  }

  .view-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    padding: var(--space-2xl);
    padding-bottom: var(--space-xl);
    border-bottom: 1px solid var(--color-border-subtle);
  }

  .header-left h1 {
    font-family: var(--font-display);
    font-size: 2.25rem;
    font-weight: 400;
    margin: 0;
    letter-spacing: -0.03em;
  }

  .subtitle {
    color: var(--color-text-tertiary);
    font-size: 0.95rem;
    margin: var(--space-xs) 0 0;
  }

  .header-actions {
    display: flex;
    gap: var(--space-sm);
  }

  .instructions-content {
    display: grid;
    grid-template-columns: 380px 1fr;
    gap: 0;
    flex: 1;
    min-height: 0;
  }

  .instructions-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
    padding: var(--space-xl);
    overflow-y: auto;
    border-right: 1px solid var(--color-border-subtle);
    background: var(--color-bg-primary);
  }

  .instruction-card {
    background: transparent;
    border: 1px solid transparent;
    border-radius: var(--radius-lg);
    padding: var(--space-md);
    cursor: pointer;
    transition: all var(--transition-normal);
    text-align: left;
    width: 100%;
    position: relative;
  }

  .instruction-card::before {
    content: '';
    position: absolute;
    left: 0;
    top: 50%;
    transform: translateY(-50%);
    width: 2px;
    height: 0;
    background: var(--color-accent-primary);
    border-radius: 1px;
    transition: height var(--transition-normal);
  }

  .instruction-card:hover {
    background: var(--color-bg-secondary);
    border-color: var(--color-border);
  }

  .instruction-card:hover::before {
    height: 24px;
  }

  .instruction-card.selected {
    background: var(--color-bg-secondary);
    border-color: var(--color-border-hover);
  }

  .instruction-card.selected::before {
    height: 32px;
    background: var(--color-accent-primary);
    box-shadow: 0 0 8px var(--color-accent-glow);
  }

  .card-header {
    display: flex;
    align-items: center;
    gap: var(--space-md);
  }

  .card-header .emoji {
    font-size: 1.5rem;
    width: 40px;
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--color-bg-tertiary);
    border-radius: var(--radius-md);
    border: 1px solid var(--color-border);
  }

  .card-info {
    flex: 1;
    min-width: 0;
  }

  .card-info h3 {
    font-family: var(--font-display);
    font-size: 1rem;
    font-weight: 400;
    margin: 0;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .category-badge {
    font-family: var(--font-mono);
    font-size: 0.65rem;
    font-weight: 500;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--color-text-muted);
  }

  .card-meta {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: var(--space-xs);
  }

  .priority {
    font-family: var(--font-mono);
    font-size: 0.7rem;
    color: var(--color-accent-primary);
  }

  .status {
    font-size: 0.85rem;
    color: var(--color-text-muted);
    opacity: 0.5;
  }

  .status.enabled {
    color: var(--color-success);
    opacity: 1;
  }

  .description {
    margin: var(--space-sm) 0 0;
    font-size: 0.85rem;
    color: var(--color-text-tertiary);
    line-height: 1.4;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .tags {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-xs);
    margin-top: var(--space-sm);
  }

  .tag {
    font-family: var(--font-mono);
    font-size: 0.65rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    background: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
    padding: 2px var(--space-sm);
    border-radius: var(--radius-sm);
    color: var(--color-text-muted);
  }

  .instruction-detail {
    background: var(--color-bg-secondary);
    padding: var(--space-2xl);
    overflow-y: auto;
    position: relative;
  }

  .instruction-detail::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 1px;
    background: linear-gradient(90deg, transparent, rgba(255,255,255,0.03), transparent);
  }

  .instruction-form h2 {
    font-family: var(--font-display);
    font-size: 1.5rem;
    font-weight: 400;
    margin: 0 0 var(--space-xl);
  }

  .form-row {
    display: flex;
    gap: var(--space-md);
    margin-bottom: var(--space-md);
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: var(--space-sm);
    margin-bottom: var(--space-md);
  }

  .form-group.flex-grow {
    flex: 1;
  }

  .form-group.emoji-picker {
    width: 80px;
  }

  .form-group.emoji-picker input {
    text-align: center;
    font-size: 1.5rem;
    padding: var(--space-sm);
  }

  .form-group.checkbox-group {
    flex-direction: row;
    align-items: center;
    gap: var(--space-sm);
  }

  .form-group.checkbox-group input {
    accent-color: var(--color-accent-primary);
  }

  .form-group label {
    font-family: var(--font-mono);
    font-size: 0.65rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: var(--color-text-muted);
  }

  .form-group input[type="text"],
  .form-group input[type="number"],
  .form-group select,
  .form-group textarea {
    background: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    padding: var(--space-md);
    color: var(--color-text-primary);
    font-size: 0.9rem;
    transition: all var(--transition-fast);
  }

  .form-group input:focus,
  .form-group select:focus,
  .form-group textarea:focus {
    outline: none;
    border-color: var(--color-border-hover);
    box-shadow: 0 0 0 3px rgba(212, 165, 116, 0.1);
  }

  .form-group textarea {
    resize: vertical;
    font-family: var(--font-mono);
    font-size: 0.85rem;
    line-height: 1.6;
  }

  .form-actions {
    display: flex;
    justify-content: flex-end;
    gap: var(--space-sm);
    margin-top: var(--space-xl);
    padding-top: var(--space-lg);
    border-top: 1px solid var(--color-border);
  }

  .preview-header {
    display: flex;
    align-items: flex-start;
    gap: var(--space-lg);
    margin-bottom: var(--space-xl);
  }

  .emoji.large {
    font-size: 2.5rem;
    width: 72px;
    height: 72px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: linear-gradient(135deg, var(--color-bg-tertiary), var(--color-bg-elevated));
    border: 1px solid var(--color-border);
    border-radius: var(--radius-xl);
    box-shadow: inset 0 1px 0 rgba(255,255,255,0.03);
  }

  .preview-info h2 {
    font-family: var(--font-display);
    font-size: 1.75rem;
    font-weight: 400;
    margin: 0;
    letter-spacing: -0.02em;
  }

  .preview-info p {
    margin: var(--space-xs) 0 0;
    color: var(--color-text-tertiary);
    font-size: 0.95rem;
  }

  .preview-meta {
    display: flex;
    gap: var(--space-lg);
    padding: var(--space-md) var(--space-lg);
    background: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    margin-bottom: var(--space-xl);
  }

  .meta-item {
    font-size: 0.85rem;
    color: var(--color-text-secondary);
  }

  .meta-item.status.enabled {
    color: var(--color-success);
  }

  .preview-content h3,
  .preview-tags h3 {
    font-family: var(--font-mono);
    font-size: 0.65rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: var(--color-text-muted);
    margin: 0 0 var(--space-sm);
  }

  .preview-content pre {
    background: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
    padding: var(--space-lg);
    border-radius: var(--radius-lg);
    overflow-x: auto;
    font-family: var(--font-mono);
    font-size: 0.8rem;
    line-height: 1.6;
    white-space: pre-wrap;
    color: var(--color-text-secondary);
  }

  .preview-tags {
    margin-top: var(--space-xl);
  }

  .preview-actions {
    display: flex;
    gap: var(--space-sm);
    margin-top: var(--space-xl);
    padding-top: var(--space-xl);
    border-top: 1px solid var(--color-border);
  }

  .empty-state,
  .empty-detail {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    gap: var(--space-md);
    color: var(--color-text-tertiary);
  }

  .empty-state .emoji,
  .empty-detail .emoji {
    font-size: 3.5rem;
    filter: drop-shadow(0 0 20px var(--color-accent-glow));
    animation: emotional-pulse 3s ease-in-out infinite;
  }

  .empty-detail p {
    font-family: var(--font-display);
    font-size: 1.125rem;
  }

  /* Modal */
  .modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.85);
    backdrop-filter: blur(8px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    animation: fade-in 0.2s ease-out;
  }

  .modal {
    background: var(--color-bg-secondary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-2xl);
    padding: var(--space-2xl);
    width: 90%;
    max-width: 600px;
    max-height: 85vh;
    overflow-y: auto;
    animation: fade-in 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    box-shadow: var(--shadow-lg);
  }

  .modal h2 {
    font-family: var(--font-display);
    font-size: 1.5rem;
    font-weight: 400;
    margin: 0 0 var(--space-sm);
  }

  .modal p {
    color: var(--color-text-tertiary);
    margin: 0 0 var(--space-lg);
    font-size: 0.9rem;
  }

  .modal textarea {
    width: 100%;
    background: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    padding: var(--space-md);
    color: var(--color-text-primary);
    font-family: var(--font-mono);
    font-size: 0.8rem;
    line-height: 1.6;
    resize: vertical;
    transition: all var(--transition-fast);
  }

  .modal textarea:focus {
    outline: none;
    border-color: var(--color-border-hover);
    box-shadow: 0 0 0 3px rgba(212, 165, 116, 0.1);
  }

  .modal-actions {
    display: flex;
    justify-content: flex-end;
    gap: var(--space-sm);
    margin-top: var(--space-lg);
  }

  /* Buttons - using global styles from app.css */
  .btn {
    padding: var(--space-sm) var(--space-lg);
    border-radius: var(--radius-md);
    font-size: 0.85rem;
    font-weight: 500;
    cursor: pointer;
    transition: all var(--transition-fast);
    border: 1px solid transparent;
    display: inline-flex;
    align-items: center;
    gap: var(--space-sm);
  }

  .btn-primary {
    background: var(--color-accent-primary);
    color: var(--color-bg-primary);
    border-color: var(--color-accent-primary);
  }

  .btn-primary:hover {
    background: var(--color-accent-secondary);
    border-color: var(--color-accent-secondary);
    transform: translateY(-1px);
  }

  .btn-secondary {
    background: transparent;
    color: var(--color-text-secondary);
    border-color: var(--color-border);
  }

  .btn-secondary:hover {
    background: var(--color-bg-tertiary);
    border-color: var(--color-border-hover);
    color: var(--color-text-primary);
  }

  .btn-danger {
    background: transparent;
    color: var(--color-error);
    border-color: var(--color-border);
  }

  .btn-danger:hover {
    background: rgba(220, 100, 100, 0.1);
    border-color: var(--color-error);
  }
</style>
