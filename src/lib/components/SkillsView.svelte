<script lang="ts">
  import { skills } from '$lib/stores';
  import type { Skill, SkillDefinition, SkillType } from '$lib/types';

  let showEditModal = false;
  let editingSkill: Skill | null = null;

  let editForm = {
    name: '',
    description: '',
    icon_emoji: '⚡',
    skill_type: 'prompt' as SkillType,
    template: '',
    enabled: true,
  };

  function openEditModal(skill?: Skill) {
    if (skill) {
      editingSkill = skill;
      editForm = {
        name: skill.name,
        description: skill.description,
        icon_emoji: skill.icon_emoji,
        skill_type: skill.skill_type,
        template: skill.definition.type === 'prompt' ? skill.definition.template : '',
        enabled: skill.enabled,
      };
    } else {
      editingSkill = null;
      editForm = {
        name: '',
        description: '',
        icon_emoji: '⚡',
        skill_type: 'prompt',
        template: '',
        enabled: true,
      };
    }
    showEditModal = true;
  }

  async function handleSaveSkill() {
    if (!editForm.name.trim()) return;

    const definition: SkillDefinition = {
      type: 'prompt',
      template: editForm.template,
    };

    try {
      if (editingSkill) {
        await skills.update({
          ...editingSkill,
          name: editForm.name,
          description: editForm.description,
          icon_emoji: editForm.icon_emoji,
          skill_type: editForm.skill_type,
          definition,
          enabled: editForm.enabled,
        });
      } else {
        await skills.create({
          name: editForm.name,
          description: editForm.description,
          icon_emoji: editForm.icon_emoji,
          skill_type: editForm.skill_type,
          definition,
          enabled: editForm.enabled,
        } as any);
      }
      showEditModal = false;
    } catch (error) {
      console.error('Failed to save skill:', error);
    }
  }

  async function handleDeleteSkill(id: string) {
    if (confirm('Are you sure you want to delete this skill?')) {
      await skills.delete(id);
    }
  }

  async function toggleSkill(skill: Skill) {
    await skills.update({
      ...skill,
      enabled: !skill.enabled,
    });
  }

  function getSkillTypeIcon(type: SkillType): string {
    switch (type) {
      case 'prompt': return '📝';
      case 'tool': return '🔧';
      case 'workflow': return '🔄';
      default: return '⚡';
    }
  }
</script>

<div class="skills-view">
  <header class="view-header">
    <div class="header-content">
      <h1 class="view-title">Skills</h1>
      <p class="view-description">Define reusable prompts, tools, and workflows for your agents</p>
    </div>
    <div class="header-actions">
      <button class="btn btn-primary" onclick={() => openEditModal()}>
        + New Skill
      </button>
    </div>
  </header>

  <div class="skills-list">
    {#each $skills as skill (skill.id)}
      <div class="skill-card card" class:disabled={!skill.enabled}>
        <div class="skill-header">
          <div class="skill-icon">{skill.icon_emoji}</div>
          <div class="skill-info">
            <h3 class="skill-name">{skill.name}</h3>
            <p class="skill-description">{skill.description}</p>
          </div>
          <div class="skill-actions">
            <button
              class="btn-icon"
              class:active={skill.enabled}
              onclick={() => toggleSkill(skill)}
              title={skill.enabled ? 'Disable' : 'Enable'}
            >
              {skill.enabled ? '✓' : '○'}
            </button>
            <button class="btn-icon" onclick={() => openEditModal(skill)} title="Edit">
              ✏️
            </button>
            <button class="btn-icon" onclick={() => handleDeleteSkill(skill.id)} title="Delete">
              🗑️
            </button>
          </div>
        </div>
        <div class="skill-meta">
          <span class="badge">
            {getSkillTypeIcon(skill.skill_type)} {skill.skill_type}
          </span>
        </div>
        {#if skill.definition.type === 'prompt' && skill.definition.template}
          <div class="skill-preview">
            <pre>{skill.definition.template.slice(0, 150)}{skill.definition.template.length > 150 ? '...' : ''}</pre>
          </div>
        {/if}
      </div>
    {:else}
      <div class="empty-state">
        <div class="empty-icon">⚡</div>
        <h3>No skills yet</h3>
        <p>Create your first skill to enhance your agents</p>
        <button class="btn btn-primary" onclick={() => openEditModal()}>
          Create Skill
        </button>
      </div>
    {/each}
  </div>
</div>

<!-- Edit Modal -->
{#if showEditModal}
  <div class="modal-overlay" onclick={() => showEditModal = false} onkeydown={(e) => e.key === 'Escape' && (showEditModal = false)} role="dialog" aria-modal="true" tabindex="-1">
    <div class="modal" onclick={(e) => e.stopPropagation()} role="document">
      <h2 class="modal-title">{editingSkill ? 'Edit Skill' : 'Create New Skill'}</h2>

      <div class="form-group">
        <label for="skill-emoji">Icon</label>
        <input
          id="skill-emoji"
          type="text"
          bind:value={editForm.icon_emoji}
          maxlength="4"
          class="emoji-input"
        />
      </div>

      <div class="form-group">
        <label for="skill-name">Name</label>
        <input
          id="skill-name"
          type="text"
          bind:value={editForm.name}
          placeholder="Code Review"
        />
      </div>

      <div class="form-group">
        <label for="skill-description">Description</label>
        <input
          id="skill-description"
          type="text"
          bind:value={editForm.description}
          placeholder="What does this skill do?"
        />
      </div>

      <div class="form-group">
        <label for="skill-type">Type</label>
        <select id="skill-type" bind:value={editForm.skill_type}>
          <option value="prompt">Prompt Template</option>
          <option value="tool">Tool (coming soon)</option>
          <option value="workflow">Workflow (coming soon)</option>
        </select>
      </div>

      {#if editForm.skill_type === 'prompt'}
        <div class="form-group">
          <label for="skill-template">Prompt Template</label>
          <textarea
            id="skill-template"
            bind:value={editForm.template}
            placeholder="Enter the prompt template that will be used when this skill is activated..."
            rows="8"
          ></textarea>
        </div>
      {/if}

      <div class="form-group checkbox-group">
        <label>
          <input type="checkbox" bind:checked={editForm.enabled} />
          <span>Enabled</span>
        </label>
      </div>

      <div class="modal-actions">
        <button class="btn btn-secondary" onclick={() => showEditModal = false}>
          Cancel
        </button>
        <button class="btn btn-primary" onclick={handleSaveSkill} disabled={!editForm.name.trim()}>
          {editingSkill ? 'Save Changes' : 'Create Skill'}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .skills-view {
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

  .skills-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-md);
    max-width: 800px;
    padding: var(--space-2xl);
  }

  .skill-card {
    background: var(--color-bg-secondary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-xl);
    padding: var(--space-xl);
    transition: all var(--transition-normal);
    position: relative;
    overflow: hidden;
  }

  .skill-card::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 1px;
    background: linear-gradient(90deg, transparent, rgba(255,255,255,0.04), transparent);
  }

  .skill-card:hover {
    border-color: var(--color-border-hover);
    transform: translateY(-1px);
  }

  .skill-card.disabled {
    opacity: 0.4;
  }

  .skill-header {
    display: flex;
    align-items: flex-start;
    gap: var(--space-lg);
  }

  .skill-icon {
    font-size: 1.75rem;
    width: 52px;
    height: 52px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: linear-gradient(135deg, var(--color-bg-tertiary), var(--color-bg-elevated));
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    box-shadow: inset 0 1px 0 rgba(255,255,255,0.03);
  }

  .skill-info {
    flex: 1;
  }

  .skill-name {
    font-family: var(--font-display);
    font-size: 1.25rem;
    font-weight: 400;
    margin: 0;
    letter-spacing: -0.01em;
  }

  .skill-description {
    color: var(--color-text-tertiary);
    font-size: 0.9rem;
    margin: var(--space-xs) 0 0;
    line-height: 1.5;
  }

  .skill-actions {
    display: flex;
    gap: var(--space-xs);
    opacity: 0.5;
    transition: opacity var(--transition-fast);
  }

  .skill-card:hover .skill-actions {
    opacity: 1;
  }

  .skill-actions .btn-icon.active {
    color: var(--color-success);
  }

  .skill-meta {
    margin-top: var(--space-md);
    margin-left: calc(52px + var(--space-lg));
  }

  .skill-preview {
    margin-top: var(--space-md);
    margin-left: calc(52px + var(--space-lg));
  }

  .skill-preview pre {
    font-size: 0.75rem;
    color: var(--color-text-muted);
    white-space: pre-wrap;
    margin: 0;
    padding: var(--space-md);
    background: var(--color-bg-tertiary);
    border-radius: var(--radius-md);
    border: 1px solid var(--color-border);
  }

  .empty-state {
    text-align: center;
    padding: var(--space-3xl);
  }

  .empty-icon {
    font-size: 3.5rem;
    margin-bottom: var(--space-lg);
    filter: drop-shadow(0 0 20px var(--color-accent-glow));
    animation: emotional-pulse 3s ease-in-out infinite;
  }

  .empty-state h3 {
    font-family: var(--font-display);
    font-size: 1.5rem;
    font-weight: 400;
    margin: 0 0 var(--space-sm);
  }

  .empty-state p {
    color: var(--color-text-tertiary);
    margin-bottom: var(--space-xl);
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
    max-width: 560px;
    width: 90%;
    max-height: 85vh;
    overflow-y: auto;
    animation: fade-in 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    box-shadow: var(--shadow-lg);
  }

  .modal-title {
    font-family: var(--font-display);
    font-size: 1.5rem;
    font-weight: 400;
    margin: 0 0 var(--space-xl);
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

  .emoji-input {
    font-size: 2rem;
    text-align: center;
    padding: var(--space-sm);
    width: 80px !important;
  }

  .checkbox-group label {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    cursor: pointer;
    font-size: 0.9rem;
  }

  .checkbox-group input[type="checkbox"] {
    width: auto;
    accent-color: var(--color-accent-primary);
  }

  .modal-actions {
    display: flex;
    justify-content: flex-end;
    gap: var(--space-sm);
    margin-top: var(--space-xl);
    padding-top: var(--space-lg);
    border-top: 1px solid var(--color-border);
  }
</style>
