import { Fragment, h } from 'preact';
import { useState } from 'preact/hooks';
import axios from 'axios';

interface Props {
  loadedHandler: (state: boolean) => void;
}

const LoginForm = (props: Props) => {
  const [formState, setFormState] = useState({
    username: '',
    password: '',
  });

  const loginFormHandler = async (e: Event) => {
    e.preventDefault();

    try {
      const { status } = await axios.post('/admin/login', { ...formState });
      props.loadedHandler(true);
    } catch (error) {
      console.log(error);
    }
  };

  return (
    <>
      <form onSubmit={loginFormHandler}>
        <label htmlFor="username">
          Username:
          <input
            type="text"
            onInput={(e) => {
              let target = e.target as HTMLInputElement;
              setFormState({ ...formState, username: target.value });
            }}
            value={formState.username}
            name="username"
          />
        </label>

        <label htmlFor="password">
          Password:
          <input
            type="password"
            onInput={(e) => {
              let target = e.target as HTMLInputElement;
              setFormState({ ...formState, password: target.value });
            }}
            value={formState.password}
            name="password"
          />
        </label>

        <input type="submit"></input>
      </form>
    </>
  );
};

export default LoginForm;
