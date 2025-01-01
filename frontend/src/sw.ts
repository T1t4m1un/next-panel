/// <reference lib='webworker' />
declare const self: ServiceWorkerGlobalScope;
import { precacheAndRoute, cleanupOutdatedCaches } from 'workbox-precaching';


self.addEventListener('message', (event) => {
  if (event.data && event.data.type === 'SKIP_WAITING') {
    self.skipWaiting();
  }
});
precacheAndRoute(self.__WB_MANIFEST);
cleanupOutdatedCaches();

