import mitt from 'mitt';
import { MediaFile, TaskProc } from 'a22-shared';
import { tauriBridge } from '../bridge/tauri-bridge';

class TauriEventService {
	private eventEmitter = mitt();

	constructor() {
		tauriBridge.subscribe(this.internalEventHandler);
	}

	subscribe(handler: (payload: TaskProc.Event) => void) {
		this.eventEmitter.on('taskEvent', handler);
	}

	unsubscribe(handler: (payload: TaskProc.Event) => void) {
		this.eventEmitter.off('taskEvent', handler);
	}

	private internalEventHandler = (event: TaskProc.Event) => {
		this.eventEmitter.emit('taskEvent', event);
	}

	getSourceByUrl(url: string) {
		return tauriBridge.getSourceByUrl(url);
	}

	addSource(source: MediaFile.Data) {
		return tauriBridge.addSource(source);
	}

	getList() {
		return tauriBridge.getList();
	}

	runTask(task: TaskProc.Input) {
		return tauriBridge.runTask(task);
	}

	abortTask(taskId: string) {
		return tauriBridge.abortTask(taskId);
	}
}

export default TauriEventService;
