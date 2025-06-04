import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { UnlistenFn } from '@tauri-apps/api/event';
import type { IPCConstantsInvoke, MediaFile, TaskProc } from 'a22-shared';
import { message } from 'antd';

export namespace TauriTypes {
	export type Event = {
		task_id: string; // -> taskId
		event_type: string; // -> type
		message: string;
		payload: any;
	}
	export const mapEvent: (Event) => TaskProc.Event = (event) => ({
		taskId: event?.payload?.task_id,
		type: ('string' === typeof event?.payload?.event_type) 
		? event.payload.event_type.toLowerCase()
		: null,
		message: event?.payload?.message,
		payload: event?.payload?.payload,
	})
}

class TauriBridge {
	private listeners: Array<(event: TaskProc.Event) => void> = [];
	private unlistenFn: UnlistenFn | null = null;

	constructor() {
		this.init();		
	}

	// REVIEW - CURRENTLY NOT WORKED becouse of tauri permissions (tauri bug?)
	private async init(): Promise<void> {
		console.log('[TauriBridge] Init');
		// await new Promise(resolve => setTimeout(resolve, 1000));
		try {
			this.unlistenFn = await listen<TauriTypes.Event>('CID_ON_TASK_PROCESSOR_EVENT', (event) => {
				const ev = TauriTypes.mapEvent(event);
				console.log('[TauriBridge] Received task processor event:', ev);
				this.listeners.forEach((fn) => fn(ev));
			});
			console.log('[TauriBridge] Event listener for CID_ON_TASK_PROCESSOR_EVENT successfully set up');
		} catch (error) {
			console.error('[TauriBridge] Failed to initialize event listener:', error.split('\n').filter(Boolean));
			throw new Error(`[TauriBridge] TauriBridge init failed: ${error}`);
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

	async appSettingsGet(): Promise<any> {
		console.log('[TauriBridge][appSettingsGet]:');
		return invoke<any>('cid_app_settings_get',);
	}

	async getList(): Promise<MediaFile.Data[]> {
		return invoke<MediaFile.Data[]>('CID_GET_LIST');
	}

	async runTask(_task: TaskProc.Input): Promise<string> {
		const task = { task_type: _task.type, payload: _task.payload, };
		console.log('[TauriBridge][Run task]:', task);
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
