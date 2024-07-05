<template>
    <div>
        <h2 class="text-blue-600 text-center text-2xl mb-4">Zgłoszenia awarii:</h2>
        <div class="flex justify-between mb-4">
            <button @click="fetchReports" class="bg-blue-600 rounded text-white p-2">Odśwież</button>
            <div class="flex space-x-2">
                <button class="bg-blue-600 rounded text-white p-2">Profil</button>
                <button @click="logout" class="bg-blue-600 rounded text-white p-2">Wyloguj</button>
                <button class="bg-blue-600 rounded text-white p-2">Kategorie</button>
            </div>
        </div>
        <div class="grid gap-4">
            <div v-for="(report, index) in reports" :key="index" class="drop-shadow-xl bg-stone-300 p-4 rounded">
                <p class="font-bold">ID: {{ index }}</p>
                <p>{{ report.nazwa }}</p>
                <div v-for="(komentarz, index_k) in report.komentarze" :key="index" class="drop-shadow-xl bg-stone-300 p-4 rounded">
                    <p class="font-bold">ID: {{ index_k }}</p>
                    <p>komentarz: {{ komentarz }}</p>
                </div>
                <div class="flex space-x-2 mt-2">
                    <button class="bg-blue-600 rounded text-white p-2" @click="deleteReport(index)">Usuń</button>
                    <button class="bg-blue-600 rounded text-white p-2" @click="editReport(index)">Edytuj</button>
                </div>
                <div v-if="editingIndex === index" class="mt-2">
                    <input v-model="newComment" class="border-2 border-blue-600 p-2 w-full" type="text">
                    <button class="bg-blue-600 rounded text-white p-2 mt-2" @click="addComment()">Dodaj komentarz</button>
                    <input v-model="editedReports[index]" class="border-2 border-blue-600 p-2 w-full" type="text">
                    <button class="bg-blue-600 rounded text-white p-2 mt-2" @click="saveEdit(index)">Zapisz</button>
                </div>
            </div>
        </div>
        <div class="flex flex-col items-center mt-4">
            <input v-model="newReport" class="border-2 border-blue-600 p-4 w-full max-w-md" type="text" placeholder="Zgłoś nową awarię">
            <button class="bg-blue-600 rounded text-white p-4 mt-2" @click="addReport">Zgłoś</button>
        </div>
    </div>
</template>

<script>
import { konkurs_backend } from 'declarations/konkurs_backend/index';

export default {
    data() {
        return {
            reports: [],
            newReport: "",
            editingIndex: -1,
            editedReports: [],
            temp_comments: [],
            newComment: ""
        }
    },
    methods: {
        async addComment() {
            this.temp_comments.push(this.newComment);
            this.newComment = "";
        },
        async addReport() {
            await konkurs_backend.dodaj_awarie(this.newReport);
            this.newReport = "";
            await this.fetchReports();
        },
        async deleteReport(index) {
            await konkurs_backend.usun_awarie(index);
            await this.fetchReports();
        },
        async editReport(index) {
            this.temp_comments = [];
            this.editingIndex = index;
            this.editedReports[index] = this.reports[index].nazwa;
        },
        async saveEdit(index) {
            await konkurs_backend.edytuj_awarie(index, this.editedReports[index]);
            for (const comment of this.temp_comments) {
                await konkurs_backend.dodaj_komentarz(index, comment);
            }
            this.temp_comments = [];
            this.editingIndex = -1;
            await this.fetchReports();
        },
        async fetchReports() {
            this.reports = await konkurs_backend.odczytaj_awarie();
            this.editedReports = new Array(this.reports.length).fill("");
        },
        async logout() {
            this.$parent.logged = false;
        }
    },
    async mounted(){
        await this.fetchReports();
    }
}
</script>