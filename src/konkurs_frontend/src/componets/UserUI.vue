

<template>
    <div>
        <button v-if="this.role == 'admin'" class="bg-blue-600 rounded text-white p-2 mt-2" @click="show_report">Wyświetl awarie</button>
        <button v-if="true" class="bg-blue-600 rounded text-white p-2 mt-2" @click="">Dodaj awarie</button>
    </div>
    <div v-if="this.role == 'admin'">
        <div v-if="this.show_value == true" class="grid gap-4">
            <div v-for="(report, index) in reports" :key="index" class="drop-shadow-xl bg-stone-300 p-4 rounded">
                <p>{{ report.nazwa }} <button @click="hide_report(index)">Wybierz</button> </p>
            </div>
        </div>
        <div v-else>
            <p>{{ report.nazwa }}</p>
            
            <div v-for="(komentarz, index) in report.komentarze" :key="index" class="drop-shadow-xl bg-stone-300 p-4 rounded">
                <p>komentarz: {{ komentarz }}</p>
            </div>
        </div>
    </div>

</template>

<script>
import { konkurs_backend } from 'declarations/konkurs_backend/index';

class Awaria {
    constructor(nazwa, komentarze = []) {
        this.nazwa = nazwa;
        this.komentarze = komentarze;
    }
}

export default {
    data() {
        return {
            role: "",
            show_value: false,
            reports: [],
            report: new Awaria("Wybierz awarie")
        };
    },
    methods: {
        async show_role() {
            await konkurs_backend.pokaz_role(this.$parent.username).then((response) => {
                if (response != "brak uzytkownika") {
                    this.role = response;
                }
            });
        },
        async show_report() {
            await this.fetchReports();
            this.show_value = true;
        },
        async hide_report(index) {
            this.show_value = false;
            this.report = this.reports[index];
            this.reports = [];
        },
        async fetchReports() {
            this.reports = await konkurs_backend.odczytaj_awarie();
        }
    },
    async mounted(){
        await this.show_role();
    }
}
</script>