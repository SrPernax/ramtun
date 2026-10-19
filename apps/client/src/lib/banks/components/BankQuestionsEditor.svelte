<script lang="ts" module>
	export type DraftQuestion = {
		key: string
		prompt: string
		options: string[]
		answerIndex: number
		justification: string
	}

	export const emptyDraftQuestion = (): DraftQuestion => ({
		key: crypto.randomUUID(),
		prompt: "",
		options: ["", ""],
		answerIndex: 0,
		justification: "",
	})
</script>

<script lang="ts">
	import { Plus, Trash2 } from "lucide-svelte"
	import {
		MAX_OPTIONS_PER_QUESTION,
		MAX_QUESTIONS_PER_BANK,
		MIN_OPTIONS_PER_QUESTION,
	} from "$lib/banks/banks.dtos"
	import MarkdownEditor from "$lib/shared/components/MarkdownEditor.svelte"

	interface BankQuestionsEditorProps {
		questions: DraftQuestion[]
		disabled?: boolean
	}

	let { questions = $bindable(), disabled = false }: BankQuestionsEditorProps = $props()

	const addQuestion = () => {
		questions.push(emptyDraftQuestion())
	}

	const removeQuestion = (index: number) => {
		questions.splice(index, 1)
	}

	const addOption = (question: DraftQuestion) => {
		question.options.push("")
	}

	// Mantiene `answerIndex` apuntando a la misma opción tras quitar una anterior.
	const removeOption = (question: DraftQuestion, optionIndex: number) => {
		question.options.splice(optionIndex, 1)

		if (optionIndex < question.answerIndex) question.answerIndex -= 1
		else if (optionIndex === question.answerIndex) question.answerIndex = 0
	}
</script>

<div class="grid gap-4">
	{#each questions as question, index (question.key)}
		<article class="panel-surface p-4 sm:p-5">
			<div class="mb-4 flex items-center justify-between gap-2">
				<h4 class="m-0 text-base font-medium text-black">Pregunta {index + 1}</h4>
				<button
					class="icon-btn icon-btn-danger"
					type="button"
					aria-label={`Eliminar pregunta ${index + 1}`}
					title="Eliminar pregunta"
					onclick={() => removeQuestion(index)}
					disabled={disabled || questions.length <= 1}
				>
					<Trash2 size={16} aria-hidden="true" />
				</button>
			</div>

			<label class="grid gap-1.5">
				<span class="text-sm text-zinc-800">Enunciado (Markdown)</span>
				<MarkdownEditor bind:value={question.prompt} />
			</label>

			<fieldset class="m-0 mt-4 grid gap-2 border-0 p-0">
				<legend class="mb-1 p-0 text-sm text-zinc-800">
					Opciones · marca la respuesta correcta
				</legend>
				{#each question.options as _, optionIndex (optionIndex)}
					<div class="flex items-center gap-2">
						<input
							class="[--check:var(--ok)]"
							type="radio"
							name={`correct-${question.key}`}
							checked={question.answerIndex === optionIndex}
							onchange={() => (question.answerIndex = optionIndex)}
							aria-label={`Opción ${optionIndex + 1} es la correcta`}
						/>
						<input
							class="input-base min-w-0 flex-1"
							bind:value={question.options[optionIndex]}
							placeholder={`Opción ${optionIndex + 1}`}
							aria-label={`Texto de la opción ${optionIndex + 1}`}
						/>
						<button
							class="icon-btn shrink-0"
							type="button"
							aria-label={`Quitar opción ${optionIndex + 1}`}
							title="Quitar opción"
							onclick={() => removeOption(question, optionIndex)}
							disabled={disabled ||
								question.options.length <= MIN_OPTIONS_PER_QUESTION}
						>
							<Trash2 size={16} aria-hidden="true" />
						</button>
					</div>
				{/each}
				<div>
					<button
						class="btn-tertiary"
						type="button"
						onclick={() => addOption(question)}
						disabled={disabled || question.options.length >= MAX_OPTIONS_PER_QUESTION}
					>
						<Plus size={14} aria-hidden="true" />
						Agregar opción
					</button>
				</div>
			</fieldset>

			<label class="mt-4 grid gap-1.5">
				<span class="text-sm text-zinc-800">Justificación (opcional)</span>
				<MarkdownEditor
					bind:value={question.justification}
					placeholder="Explica por qué la respuesta correcta lo es..."
				/>
			</label>
		</article>
	{/each}

	<div>
		<button
			class="btn-secondary flex items-center gap-1.5"
			type="button"
			onclick={addQuestion}
			disabled={disabled || questions.length >= MAX_QUESTIONS_PER_BANK}
		>
			<Plus size={16} aria-hidden="true" />
			Agregar pregunta
		</button>
	</div>
</div>
