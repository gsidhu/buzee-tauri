import * as Sentry from '@sentry/sveltekit';

setTimeout(() => {
  Sentry.init({
    dsn: "https://c80c1a4e734f24a7de3c2c9f1ff8a05e@o4506604817481728.ingest.sentry.io/4506604819644416",
  
    // We recommend adjusting this value in production, or using tracesSampler
    // for finer control
    tracesSampleRate: 1.0,
  
    // Optional: Initialize Session Replay:
    integrations: [new Sentry.Replay()],
    replaysSessionSampleRate: 0.1,
    replaysOnErrorSampleRate: 1.0,
  });
}, 10);

const myErrorHandler = ({ error, event }: { error: unknown; event: unknown }) => {
	console.error('An error occurred on the client side:', error, event);
};

export const handleError = Sentry.handleErrorWithSentry(myErrorHandler);
