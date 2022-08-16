import { Fragment, h, render } from 'preact';
import { useState } from 'preact/hooks';

import LoginForm from './components/loginForm';

const App = () => {
  const [loaded, setLoaded] = useState(false);

  const loadedHandler = (state: boolean) => {
    setLoaded(state);
  };

  return (
    <>
      {!loaded && (
        <>
          <LoginForm loadedHandler={loadedHandler} />
        </>
      )}
      {loaded && (
        <>
          <h1>Admin Page</h1>
        </>
      )}
    </>
  );
};

render(<App />, document.getElementById('root'));
