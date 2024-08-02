import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { VUIKitLoader } from 'athnuikit';

import App from '@app/App.vue'
import router from '@app/router'

import 'athnuikit/dist/style.css';

import 'primevue/resources/themes/saga-blue/theme.css';
import 'primevue/resources/primevue.min.css';
import 'primeicons/primeicons.css';

import PrimeVue from 'primevue/config';

import SpeedDial from 'primevue/speeddial';
import SplitButton from 'primevue/splitbutton';
import ToastService from 'primevue/toastservice';
import Tooltip from 'primevue/tooltip';

import FontAwesomeIcon from '@app/font-awesome.loader';

const app = createApp(App);

app.use(createPinia());
app.use(router);
app.use(PrimeVue);
app.use(ToastService);

app.component('SplitButton', SplitButton);
app.component('SpeedDial', SpeedDial);
app.component('font-awesome-icon', FontAwesomeIcon);

app.directive('tooltip', Tooltip);

new VUIKitLoader().load(app);

app.mount('#app');