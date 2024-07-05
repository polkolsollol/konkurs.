<script setup>
import DeviceReports from '../componets/DeviceReports.vue'
import Rejestracja from '../componets/Rejestracja.vue'
</script>

<template>
  <div>
    <h1>Witaj w aplikacji konkursowej</h1>
    <div v-if="logging">
      <div v-if="!logged">
        <h2>Zaloguj się</h2>
        <form action="#" @submit.prevent="login">
          <div>
            <label for="username">Nazwa użytkownika:</label>
            <input v-model="username" type="text">
          </div>
          <div>
            <label for="password">Hasło:</label>
            <input v-model="password" type="text">
          </div>
          <button type="submit">Zaloguj</button>
        </form>
        <p v-if="errorlog" class="error">{{ errorlog }}</p>
      </div>
      <div v-else>
        <p>Zalogowano jako {{ username }}</p>
        <DeviceReports />
      </div>
      <div v-if="!logged">
        <button @click="toggleLogin">Zarejestruj się</button>
      </div>
      
    </div>
    <div v-else>
      <div v-if="!logged">
        <p>Zarejestruj się.</p>
        <Rejestracja />
        <button @click="toggleLogin">Zaloguj się</button>
      </div>
      
    </div>
  </div>
</template>

<script>
import { konkurs_backend } from 'declarations/konkurs_backend/index';

export default {
    data() {
        return {
            username: "",
            password: "",
            logged: false,
            logging: true,
            errorlog: ""
        };
    },
    methods: {
        async login() {
            
            await konkurs_backend.zaloguj(this.username, this.password).then((response) => {
                if (response == true) {
                    this.logged = true;
                } else {
                    this.errorlog = 'Nieprawidłowa nazwa użytkownika lub hasło ';
                }
            });
        },
        async toggleLogin() {
          this.logging = !this.logging;
        }
    }
}
</script>