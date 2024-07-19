<template>
    <div>
      <h2>Formularz rejestracji</h2>
      <form @submit.prevent="register">
        <div>
          <label>Nazwa użytkownika:</label>
          <input type="text" v-model="username" required>
        </div>
        <div>
          <label>Hasło:</label>
          <input type="password" v-model="password" required>
        </div>
        <div>
          <button type="submit">Zarejestruj</button>
        </div>
      </form>
    </div>
  </template>
  
  <script>
  import { konkurs_backend } from 'declarations/konkurs_backend/index';
  
  export default {
    data() {
      return {
        username: "",
        password: ""
      };
    },
    methods: {
      async register() {
        await konkurs_backend.dodaj_uzytkownika(this.username, this.password, "user").then((response) => {
          alert(response);
        }).catch((error) => {
          console.error('Błąd podczas rejestracji:', error);
          alert('Wystąpił błąd podczas rejestracji.');
        });
      }
    }
  };
  </script>