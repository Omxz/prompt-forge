<script lang="ts">
  import { agents, skills, instructions } from '$lib/stores';
  import type { Agent, Personality } from '$lib/types';

  let showImportModal = false;
  let showEditModal = false;
  let importText = '';
  let editingAgent: Agent | null = null;
  let importError = '';

  // Form fields for editing
  let editForm = {
    name: '',
    description: '',
    avatar_emoji: '',
    system_prompt: '',
    skills: [] as string[],
    instructions: [] as string[],
    tags: [] as string[],
    personality: {
      tone: 'friendly',
      verbosity: 'balanced',
      creativity: 0.5,
      formality: 0.5,
      traits: [] as string[],
    } as Personality,
  };

  let newTrait = '';
  let newTag = '';

  async function handleImport() {
    if (!importText.trim()) return;

    try {
      importError = '';
      await agents.importFromText(importText);
      showImportModal = false;
      importText = '';
    } catch (error) {
      importError = String(error);
    }
  }

  function openEditModal(agent?: Agent) {
    if (agent) {
      editingAgent = agent;
      editForm = {
        name: agent.name,
        description: agent.description,
        avatar_emoji: agent.avatar_emoji,
        system_prompt: agent.system_prompt,
        skills: [...agent.skills],
        instructions: [...agent.instructions],
        tags: [...agent.tags],
        personality: { ...agent.personality, traits: [...agent.personality.traits] },
      };
    } else {
      editingAgent = null;
      editForm = {
        name: '',
        description: '',
        avatar_emoji: '🤖',
        system_prompt: '',
        skills: [],
        instructions: [],
        tags: [],
        personality: {
          tone: 'friendly',
          verbosity: 'balanced',
          creativity: 0.5,
          formality: 0.5,
          traits: [],
        },
      };
    }
    showEditModal = true;
  }

  async function handleSaveAgent() {
    if (!editForm.name.trim()) return;

    try {
      if (editingAgent) {
        await agents.update({
          ...editingAgent,
          ...editForm,
        });
      } else {
        await agents.create(editForm as any);
      }
      showEditModal = false;
    } catch (error) {
      console.error('Failed to save agent:', error);
    }
  }

  async function handleDeleteAgent(id: string) {
    if (confirm('Are you sure you want to delete this agent?')) {
      await agents.delete(id);
    }
  }

  async function handleExport(agent: Agent) {
    try {
      const markdown = await agents.exportToMarkdown(agent.id);
      if (markdown) {
        await navigator.clipboard.writeText(markdown);
        alert('Agent exported to clipboard!');
      }
    } catch (error) {
      console.error('Failed to export agent:', error);
    }
  }

  function addTrait() {
    if (newTrait.trim() && !editForm.personality.traits.includes(newTrait.trim())) {
      editForm.personality.traits = [...editForm.personality.traits, newTrait.trim()];
      newTrait = '';
    }
  }

  function removeTrait(trait: string) {
    editForm.personality.traits = editForm.personality.traits.filter(t => t !== trait);
  }

  function addTag() {
    if (newTag.trim() && !editForm.tags.includes(newTag.trim())) {
      editForm.tags = [...editForm.tags, newTag.trim()];
      newTag = '';
    }
  }

  function removeTag(tag: string) {
    editForm.tags = editForm.tags.filter(t => t !== tag);
  }

  function toggleSkill(skillId: string) {
    if (editForm.skills.includes(skillId)) {
      editForm.skills = editForm.skills.filter(s => s !== skillId);
    } else {
      editForm.skills = [...editForm.skills, skillId];
    }
  }

  function toggleInstruction(instructionId: string) {
    if (editForm.instructions.includes(instructionId)) {
      editForm.instructions = editForm.instructions.filter(i => i !== instructionId);
    } else {
      editForm.instructions = [...editForm.instructions, instructionId];
    }
  }

  function getSkillName(skillId: string): string {
    const skill = $skills.find(s => s.id === skillId);
    return skill?.name || skillId;
  }

  function getInstructionTitle(instructionId: string): string {
    const instruction = $instructions.find(i => i.id === instructionId);
    return instruction?.name || instructionId;
  }

  function formatLastUsed(lastUsedAt: string | null): string {
    if (!lastUsedAt) return 'Never used';
    const date = new Date(lastUsedAt);
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffMins = Math.floor(diffMs / 60000);
    const diffHours = Math.floor(diffMs / 3600000);
    const diffDays = Math.floor(diffMs / 86400000);

    if (diffMins < 1) return 'Just now';
    if (diffMins < 60) return `${diffMins}m ago`;
    if (diffHours < 24) return `${diffHours}h ago`;
    if (diffDays === 1) return 'Yesterday';
    if (diffDays < 30) return `${diffDays}d ago`;
    return date.toLocaleDateString();
  }
</script>

<div class="agents-view">
  <header class="view-header">
    <div class="header-content">
      <h1 class="view-title">Agents</h1>
      <p class="view-description">Create and manage AI agents with custom skills and instructions</p>
    </div>
    <div class="header-actions">
      <button class="btn btn-secondary" onclick={() => showImportModal = true}>
        📋 Import
      </button>
      <button class="btn btn-primary" onclick={() => openEditModal()}>
        + New Agent
      </button>
    </div>
  </header>

  <div class="agents-grid">
    {#each $agents as agent (agent.id)}
      <div class="agent-card card">
        <div class="agent-card-header">
          <span class="agent-card-avatar">{agent.avatar_emoji}</span>
          <div class="agent-card-actions">
            <button class="btn-icon" onclick={() => handleExport(agent)} title="Export">
              📤
            </button>
            <button class="btn-icon" onclick={() => openEditModal(agent)} title="Edit">
              ✏️
            </button>
            {#if agent.id !== 'default'}
              <button class="btn-icon" onclick={() => handleDeleteAgent(agent.id)} title="Delete">
                🗑️
              </button>
            {/if}
          </div>
        </div>
        <h3 class="agent-card-name">{agent.name}</h3>
        <p class="agent-card-description">{agent.description}</p>

        <div class="agent-attachments">
          {#if agent.skills.length > 0}
            <div class="attachment-group">
              <span class="attachment-label">⚡ Skills:</span>
              <span class="attachment-count">{agent.skills.length}</span>
            </div>
          {/if}
          {#if agent.instructions.length > 0}
            <div class="attachment-group">
              <span class="attachment-label">📋 Instructions:</span>
              <span class="attachment-count">{agent.instructions.length}</span>
            </div>
          {/if}
        </div>

        <div class="agent-card-tags">
          {#each agent.tags.slice(0, 3) as tag}
            <span class="badge">{tag}</span>
          {/each}
          {#if agent.tags.length > 3}
            <span class="badge badge-muted">+{agent.tags.length - 3}</span>
          {/if}
        </div>

        <div class="agent-card-meta">
          <span class="meta-item">🎯 {agent.personality.tone}</span>
          <span class="meta-item">📝 {agent.personality.verbosity}</span>
        </div>

        <div class="agent-card-usage">
          <span class="usage-stat" title="Times used">📊 {agent.usage_count || 0}</span>
          <span class="usage-stat usage-last" title="Last used">{formatLastUsed(agent.last_used_at)}</span>
        </div>
      </div>
    {/each}
  </div>
</div>

<!-- Import Modal -->
{#if showImportModal}
  <div class="modal-overlay" onclick={() => showImportModal = false} onkeydown={(e) => e.key === 'Escape' && (showImportModal = false)} role="dialog" aria-modal="true" tabindex="-1">
    <div class="modal" onclick={(e) => e.stopPropagation()} role="document">
      <h2 class="modal-title">Import Agent from Text</h2>
      <p class="modal-description">
        Paste a markdown or YAML agent definition. Supports CLAUDE.md format.
      </p>

      <div class="form-group">
        <label for="import-text">Agent Definition</label>
        <textarea
          id="import-text"
          bind:value={importText}
          placeholder={`# My Agent

## Description
A helpful assistant...

## Personality
Tone: friendly
- helpful
- creative

## System Prompt
You are...`}
          rows="15"
        ></textarea>
      </div>

      {#if importError}
        <p class="error-message">{importError}</p>
      {/if}

      <div class="modal-actions">
        <button class="btn btn-secondary" onclick={() => showImportModal = false}>
          Cancel
        </button>
        <button class="btn btn-primary" onclick={handleImport} disabled={!importText.trim()}>
          Import Agent
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Edit Modal -->
{#if showEditModal}
  <div class="modal-overlay" onclick={() => showEditModal = false} onkeydown={(e) => e.key === 'Escape' && (showEditModal = false)} role="dialog" aria-modal="true" tabindex="-1">
    <div class="modal modal-large" onclick={(e) => e.stopPropagation()} role="document">
      <h2 class="modal-title">{editingAgent ? 'Edit Agent' : 'Create New Agent'}</h2>

      <div class="form-grid">
        <div class="form-group">
          <label for="agent-emoji">Avatar Emoji</label>
          <input
            id="agent-emoji"
            type="text"
            bind:value={editForm.avatar_emoji}
            maxlength="4"
            class="emoji-input"
          />
        </div>

        <div class="form-group">
          <label for="agent-name">Name</label>
          <input
            id="agent-name"
            type="text"
            bind:value={editForm.name}
            placeholder="My Assistant"
          />
        </div>

        <div class="form-group full-width">
          <label for="agent-description">Description</label>
          <input
            id="agent-description"
            type="text"
            bind:value={editForm.description}
            placeholder="A helpful AI assistant that..."
          />
        </div>

        <div class="form-group full-width">
          <label for="agent-prompt">System Prompt</label>
          <textarea
            id="agent-prompt"
            bind:value={editForm.system_prompt}
            placeholder="You are a helpful AI assistant..."
            rows="5"
          ></textarea>
        </div>

        <div class="form-group">
          <label for="agent-tone">Tone</label>
          <select id="agent-tone" bind:value={editForm.personality.tone}>
            <option value="friendly">Friendly</option>
            <option value="professional">Professional</option>
            <option value="casual">Casual</option>
            <option value="formal">Formal</option>
            <option value="humorous">Humorous</option>
          </select>
        </div>

        <div class="form-group">
          <label for="agent-verbosity">Verbosity</label>
          <select id="agent-verbosity" bind:value={editForm.personality.verbosity}>
            <option value="concise">Concise</option>
            <option value="balanced">Balanced</option>
            <option value="detailed">Detailed</option>
          </select>
        </div>

        <!-- Skills Selection -->
        <div class="form-group full-width">
          <label>Attached Skills</label>
          <div class="selection-grid">
            {#each $skills as skill}
              <button
                class="selection-item"
                class:selected={editForm.skills.includes(skill.id)}
                onclick={() => toggleSkill(skill.id)}
              >
                <span class="selection-icon">⚡</span>
                <span class="selection-name">{skill.name}</span>
                {#if editForm.skills.includes(skill.id)}
                  <span class="selection-check">✓</span>
                {/if}
              </button>
            {/each}
            {#if $skills.length === 0}
              <p class="empty-hint">No skills available. Create some in the Skills tab.</p>
            {/if}
          </div>
        </div>

        <!-- Instructions Selection -->
        <div class="form-group full-width">
          <label>Attached Instructions</label>
          <div class="selection-grid">
            {#each $instructions as instruction}
              <button
                class="selection-item"
                class:selected={editForm.instructions.includes(instruction.id)}
                onclick={() => toggleInstruction(instruction.id)}
              >
                <span class="selection-icon">{instruction.category === 'code_style' ? '💻' : instruction.category === 'testing' ? '🧪' : '📋'}</span>
                <span class="selection-name">{instruction.name}</span>
                {#if editForm.instructions.includes(instruction.id)}
                  <span class="selection-check">✓</span>
                {/if}
              </button>
            {/each}
            {#if $instructions.length === 0}
              <p class="empty-hint">No instructions available. Create some in the Instructions tab.</p>
            {/if}
          </div>
        </div>

        <!-- Tags -->
        <div class="form-group full-width">
          <label>Tags</label>
          <div class="tags-input">
            <input
              type="text"
              bind:value={newTag}
              placeholder="Add a tag..."
              onkeydown={(e) => e.key === 'Enter' && addTag()}
            />
            <button class="btn btn-secondary" onclick={addTag}>Add</button>
          </div>
          <div class="tags-list">
            {#each editForm.tags as tag}
              <span class="badge badge-removable">
                {tag}
                <button class="remove-btn" onclick={() => removeTag(tag)}>×</button>
              </span>
            {/each}
          </div>
        </div>

        <!-- Personality Traits -->
        <div class="form-group full-width">
          <label>Personality Traits</label>
          <div class="traits-input">
            <input
              type="text"
              bind:value={newTrait}
              placeholder="Add a trait..."
              onkeydown={(e) => e.key === 'Enter' && addTrait()}
            />
            <button class="btn btn-secondary" onclick={addTrait}>Add</button>
          </div>
          <div class="traits-list">
            {#each editForm.personality.traits as trait}
              <span class="badge badge-removable">
                {trait}
                <button class="remove-btn" onclick={() => removeTrait(trait)}>×</button>
              </span>
            {/each}
          </div>
        </div>
      </div>

      <div class="modal-actions">
        <button class="btn btn-secondary" onclick={() => showEditModal = false}>
          Cancel
        </button>
        <button class="btn btn-primary" onclick={handleSaveAgent} disabled={!editForm.name.trim()}>
          {editingAgent ? 'Save Changes' : 'Create Agent'}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .agents-view {
    height: 100%;
    overflow-y: auto;
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

  .header-content {
    max-width: 500px;
  }

  .view-title {
    font-family: var(--font-display);
    font-size: 2.25rem;
    font-weight: 400;
    margin: 0 0 var(--space-sm);
    letter-spacing: -0.03em;
  }

  .view-description {
    color: var(--color-text-tertiary);
    font-size: 0.95rem;
    line-height: 1.5;
    margin: 0;
  }

  .header-actions {
    display: flex;
    gap: var(--space-sm);
  }

  .agents-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
    gap: var(--space-lg);
    padding: var(--space-2xl);
  }

  .agent-card {
    display: flex;
    flex-direction: column;
    gap: var(--space-md);
    background: var(--color-bg-secondary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-xl);
    padding: var(--space-xl);
    transition: all var(--transition-normal);
    position: relative;
    overflow: hidden;
  }

  .agent-card::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 1px;
    background: linear-gradient(90deg, transparent, rgba(255,255,255,0.04), transparent);
  }

  .agent-card:hover {
    border-color: var(--color-border-hover);
    transform: translateY(-2px);
    box-shadow: var(--shadow-md);
  }

  .agent-card-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
  }

  .agent-card-avatar {
    font-size: 2.75rem;
    filter: drop-shadow(0 0 12px var(--color-accent-glow));
  }

  .agent-card-actions {
    display: flex;
    gap: var(--space-xs);
    opacity: 0.5;
    transition: opacity var(--transition-fast);
  }

  .agent-card:hover .agent-card-actions {
    opacity: 1;
  }

  .agent-card-name {
    font-family: var(--font-display);
    font-size: 1.35rem;
    font-weight: 400;
    margin: 0;
    letter-spacing: -0.02em;
  }

  .agent-card-description {
    color: var(--color-text-tertiary);
    font-size: 0.9rem;
    margin: 0;
    line-height: 1.5;
  }

  .agent-attachments {
    display: flex;
    gap: var(--space-lg);
    margin-top: var(--space-xs);
  }

  .attachment-group {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    font-family: var(--font-mono);
    font-size: 0.7rem;
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .attachment-count {
    background: var(--color-accent-glow);
    color: var(--color-accent-primary);
    padding: 0.15rem 0.5rem;
    border-radius: var(--radius-sm);
    font-weight: 600;
  }

  .agent-card-tags {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-xs);
    margin-top: var(--space-sm);
  }

  .badge-muted {
    background: var(--color-bg-tertiary);
    color: var(--color-text-muted);
  }

  .agent-card-meta {
    display: flex;
    gap: var(--space-lg);
    font-family: var(--font-mono);
    font-size: 0.7rem;
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.02em;
    margin-top: auto;
    padding-top: var(--space-md);
    border-top: 1px solid var(--color-border);
  }

  .meta-item {
    display: flex;
    align-items: center;
    gap: var(--space-xs);
  }

  .agent-card-usage {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 0.7rem;
    color: var(--color-text-muted);
    margin-top: var(--space-sm);
    padding-top: var(--space-sm);
    border-top: 1px dashed var(--color-border);
  }

  .usage-stat {
    display: flex;
    align-items: center;
    gap: var(--space-xs);
    font-family: var(--font-mono);
  }

  .usage-last {
    font-style: italic;
    opacity: 0.8;
  }

  /* Modal Styles */
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
    max-width: 520px;
    width: 90%;
    max-height: 85vh;
    overflow-y: auto;
    animation: fade-in 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    box-shadow: var(--shadow-lg);
  }

  .modal-large {
    max-width: 720px;
  }

  .modal-title {
    font-family: var(--font-display);
    font-size: 1.5rem;
    font-weight: 400;
    margin: 0 0 var(--space-sm);
  }

  .modal-description {
    color: var(--color-text-tertiary);
    margin-bottom: var(--space-xl);
    font-size: 0.9rem;
  }

  .form-group {
    margin-bottom: var(--space-lg);
  }

  .form-group label {
    display: block;
    font-family: var(--font-mono);
    font-size: 0.65rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: var(--color-text-muted);
    margin-bottom: var(--space-sm);
  }

  .form-group input,
  .form-group textarea,
  .form-group select {
    width: 100%;
  }

  .form-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--space-lg);
  }

  .full-width {
    grid-column: 1 / -1;
  }

  .emoji-input {
    font-size: 2rem;
    text-align: center;
    padding: var(--space-sm);
    width: 80px !important;
  }

  .selection-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
    gap: var(--space-sm);
  }

  .selection-item {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-sm) var(--space-md);
    background: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    cursor: pointer;
    transition: all var(--transition-fast);
    font-size: 0.85rem;
  }

  .selection-item:hover {
    background: var(--color-bg-hover);
    border-color: var(--color-border-hover);
  }

  .selection-item.selected {
    border-color: var(--color-accent-muted);
    background: var(--color-accent-glow);
    color: var(--color-accent-primary);
  }

  .selection-icon {
    font-size: 1.125rem;
  }

  .selection-name {
    flex: 1;
    text-align: left;
  }

  .selection-check {
    color: var(--color-accent-primary);
    font-weight: 600;
  }

  .empty-hint {
    color: var(--color-text-muted);
    font-size: 0.85rem;
    font-style: italic;
    margin: 0;
    grid-column: 1 / -1;
  }

  .tags-input,
  .traits-input {
    display: flex;
    gap: var(--space-sm);
    margin-bottom: var(--space-sm);
  }

  .tags-input input,
  .traits-input input {
    flex: 1;
  }

  .tags-list,
  .traits-list {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-xs);
  }

  .badge-removable {
    display: inline-flex;
    align-items: center;
    gap: var(--space-xs);
  }

  .remove-btn {
    background: none;
    border: none;
    color: inherit;
    cursor: pointer;
    padding: 0;
    font-size: 1rem;
    opacity: 0.7;
    transition: opacity var(--transition-fast);
  }

  .remove-btn:hover {
    opacity: 1;
  }

  .modal-actions {
    display: flex;
    justify-content: flex-end;
    gap: var(--space-sm);
    margin-top: var(--space-xl);
    padding-top: var(--space-lg);
    border-top: 1px solid var(--color-border);
  }

  .error-message {
    color: var(--color-error);
    font-size: 0.85rem;
    margin-top: var(--space-sm);
    font-family: var(--font-mono);
  }
</style>
