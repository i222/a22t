// ui-react/src/contexts/BridgeServiceContext.tsx
import React, { createContext, useContext, ReactNode } from 'react';
import TauriEventService from '../services/tauri-event-service';

const BridgeServiceContext = createContext<TauriEventService | null>(null);

export const useBridgeService = (): TauriEventService => {
	const context = useContext(BridgeServiceContext);
	if (!context) {
		throw new Error('useBridgeService must be used within a BridgeServiceProvider');
	}
	return context;
};

interface BridgeServiceProviderProps {
	children: ReactNode;
}

export const BridgeServiceProvider: React.FC<BridgeServiceProviderProps> = ({ children }) => {
	// Creating a single instance of the service for the entire app
	// const bridgeService = new EventService();

	const bridgeService = new TauriEventService();

	return (
		<BridgeServiceContext.Provider value={bridgeService}>
			{children}
		</BridgeServiceContext.Provider>
	);
};
