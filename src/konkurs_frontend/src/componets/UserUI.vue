

<template>
    <div>
        <button class="bg-blue-600 rounded text-white p-2 mt-2" @click="show_report">Wyświetl awarie</button>
        <button class="bg-blue-600 rounded text-white p-2 mt-2" @click="add_report_show">Dodaj awarie</button>
    </div>
    <div v-if="this.showing == true">
        <div v-if="this.show_value == true" class="grid gap-4">
            <div v-for="(report, index) in reports" :key="index" class="drop-shadow-xl bg-stone-300 p-4 rounded">
                <p v-if="this.$parent.username == report.owner.username || this.role == 'admin'">{{ report.nazwa }} <button @click="hide_report(index)">Wybierz</button> </p>
            </div>
        </div>
        <div v-else>
            <p> Awaria: {{ report.nazwa }}</p>
            <p>Włąściciel: {{ report.owner.username }}</p>
            <p>Kontakt (gmail): {{ report.owner.gmail }}</p>
            
            <div v-for="(komentarz, index) in report.komentarze" :key="index" class="drop-shadow-xl bg-stone-300 p-4 rounded">
                <p>komentarz: {{ komentarz }}  
                    <button v-if="editingIndex === this.report_index" class="bg-blue-600 rounded text-white p-2 mt-2" @click="">Usuń</button>
                    <button v-if="editingIndex === this.report_index" class="bg-blue-600 rounded text-white p-2 mt-2" @click="">Edytuj</button>
                </p>
            </div>

            <div class="flex space-x-2 mt-2">
            <button class="bg-blue-600 rounded text-white p-2" @click="delete_report(this.report_index)">Usuń</button>
            <button class="bg-blue-600 rounded text-white p-2" @click="edit_report(this.report_index)">Edytuj</button>
            </div>
            <div v-if="editingIndex === this.report_index" class="mt-2">
                <input v-model="newComment" class="border-2 border-blue-600 p-2 w-full" type="text">
                <button class="bg-blue-600 rounded text-white p-2 mt-2" @click="addComment()">Dodaj komentarz</button><p></p>
                <input v-model="editedReports[this.report_index]" class="border-2 border-blue-600 p-2 w-full" type="text">
                <button class="bg-blue-600 rounded text-white p-2 mt-2" @click="saveEdit(this.report_index)">Zapisz</button>
                <button class="bg-blue-600 rounded text-white p-2 mt-2" @click="declineEdit(this.report_index)">Anuluj</button>
            </div>
        </div>
    </div>
    <div v-if="this.adding == true">
        <div class="flex flex-col items-center mt-4">
            <input v-model="newReport" class="border-2 border-blue-600 p-4 w-full max-w-md" type="text" placeholder="Zgłoś nową awarię">
            <button class="bg-blue-600 rounded text-white p-4 mt-2" @click="add_report">Zgłoś</button>
        </div>
    </div>

</template>

<script>
import { konkurs_backend } from 'declarations/konkurs_backend/index';

class Awaria {
    constructor(nazwa, komentarze = [], owner) {
        this.nazwa = nazwa;
        this.komentarze = komentarze;
        this.owner = owner;
    }
}

export default {
    data() {
        return {
            role: "",
            newReport: "",
            show_value: false,
            adding: false,
            showing: false,
            reports: [],
            report: new Awaria("", this.$parent.user),
            adding: false,
            editingIndex: -1,
            editedReports: [],
            temp_comments: [],
            newComment: "",
            report_index: -1
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
            this.report_index = -1;
            this.showing = true;
            this.adding = false;
            await this.fetchReports();
            this.show_value = true;
        },
        async hide_report(index) {
            this.report_index = index;
            this.show_value = false;
            this.report = this.reports[index];
            this.reports = [];
        },
        async add_report_show() {
            this.showing = false;
            this.adding = true;
        },
        async delete_report(index) {
            this.report_index = -1;
            await konkurs_backend.usun_awarie(index);
            this.report = new Awaria("", this.$parent.user);
            this.showing = false;
            this.adding = false;
        },
        async edit_report(index) {
            this.editedReports[this.report_index] = this.report.nazwa;
            this.temp_comments = [];
            this.editingIndex = index;
        },
        async addComment() {
            this.temp_comments.push(this.newComment);
            this.newComment = "";
        },
        async saveEdit(index) {
            await konkurs_backend.edytuj_awarie(index, this.editedReports[index]);
            for (const comment of this.temp_comments) {
                await konkurs_backend.dodaj_komentarz(index, comment);
            }
            await this.fetchReports();
            this.report = this.reports[this.report_index];
            this.reports = [];
            this.temp_comments = [];
            this.editingIndex = -1;
        },
        async declineEdit(index) {
            await this.fetchReports();
            this.report = this.reports[this.report_index];
            this.reports = [];
            this.temp_comments = [];
            this.editingIndex = -1;
        },
        async add_report() {
            await konkurs_backend.dodaj_awarie(this.newReport, this.$parent.user);
            this.newReport = "";
        },
        async fetchReports() {
            this.reports = await konkurs_backend.odczytaj_awarie();
            this.editedReports = new Array(this.reports.length).fill("");
        }
    },
    async mounted(){
        await this.show_role();
    }
}
</script>