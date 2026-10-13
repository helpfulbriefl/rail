import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { MotionConfig } from 'motion/react';
import '@fontsource-variable/geist';
import '@fontsource-variable/geist-mono';
import './styles/tokens.css';
import './styles/app.css';
import { AppCtx, useBoot } from './ui/useApp';
import { Rail } from './rail/Rail';
import { Flyout } from './flyout/Flyout';
import { SettingsView } from './settings/Settings';

function Root() {
  const app = useBoot();
  if (!app) return null;
  const View = app.api.label === 'flyout' ? Flyout : app.api.label === 'settings' ? SettingsView : Rail;
  document.documentElement.dataset.window = app.api.label;
  return (
    <AppCtx.Provider value={app}>
      <MotionConfig reducedMotion={app.config.settings.reduceMotion ? 'always' : 'user'}>
        <View />
      </MotionConfig>
    </AppCtx.Provider>
  );
}

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <Root />
  </StrictMode>,
);
