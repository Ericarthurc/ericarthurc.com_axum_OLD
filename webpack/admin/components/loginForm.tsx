import { Fragment, h } from 'preact';
import { useState } from 'preact/hooks';
import axios from 'axios';

interface Props {
  loadedHandler: (state: boolean) => void;
}

const LoginForm = (props: Props) => {
  const [formState, setFormState] = useState({
    password: '',
    pin: '',
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

        <label htmlFor="pin">
          Pin:
          <input
            type="password"
            onInput={(e) => {
              let target = e.target as HTMLInputElement;
              setFormState({ ...formState, pin: target.value });
            }}
            value={formState.pin}
            name="pin"
          />
        </label>

        <input type="submit"></input>
      </form>
    </>
  );
};

export default LoginForm;
