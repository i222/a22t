import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { UnlistenFn } from '@tauri-apps/api/event';
import type { IPCConstantsInvoke, MediaFile, TaskProc } from 'a22-shared';


class TauriBridge {
	private listeners: Array<(event: TaskProc.Event) => void> = [];
	private unlistenFn: UnlistenFn | null = null;

	constructor() {
		this.init();
	}

	// private async init() {
	// 	this.unlistenFn = await listen<TaskProc.Event>('CID_ON_TASK_PROCESSOR_EVENT', event => {
	// 		console
	// 		this.listeners.forEach(fn => fn(event.payload));
	// 	});
	// }

	// REVIEW - CURRENTLY NOT WORKED becouse of tauri permissions (tauri bug?)
	private async init(): Promise<void> {
		await new Promise(resolve => setTimeout(resolve, 1000));
    try {
      this.unlistenFn = await listen<TaskProc.Event>('CID_ON_TASK_PROCESSOR_EVENT', (event) => {
        console.log('Received task processor event:', event);
        this.listeners.forEach((fn) => fn(event.payload));
      });
      console.log('Event listener for CID_ON_TASK_PROCESSOR_EVENT successfully set up');
    } catch (error) {
      console.error('Failed to initialize event listener:', error.split('\n').filter(Boolean));
      throw new Error(`TauriBridge init failed: ${error}`);
    }
  }

	subscribe(handler: (event: TaskProc.Event) => void): void {
		if (!this.listeners.includes(handler)) {
			this.listeners.push(handler);
		}
	}

	unsubscribe(handler: (event: TaskProc.Event) => void): void {
		this.listeners = this.listeners.filter(fn => fn !== handler);
	}

	async getSourceByUrl(url: string): Promise<MediaFile.SourceFile | MediaFile.UrlInfo> {
		return invoke<MediaFile.SourceFile | MediaFile.UrlInfo>('CID_GET_SOURCE_INFO', { url });
	}

	async addSource(source: MediaFile.Data): Promise<boolean> {
		return invoke<boolean>('CID_ADD_SOURCE', { source });
	}

	async getList(): Promise<MediaFile.Data[]> {
		return invoke<MediaFile.Data[]>('CID_GET_LIST');
	}

	async runTask(_task: TaskProc.Input): Promise<string> {
		const task = { task_type: _task.type, payload: _task.payload, };
		return invoke<string>('cid_run_task', { task });
	}

	async abortTask(taskId: string): Promise<boolean> {
		return invoke<boolean>('cid_abort_task', { taskId });
	}

	async destroy() {
		if (this.unlistenFn) {
			await this.unlistenFn();
			this.unlistenFn = null;
		}
	}
}

export const tauriBridge = new TauriBridge();
