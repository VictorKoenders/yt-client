const storage = {
	login() {
		const access_token = localStorage.getItem('access_token');
		const refresh_token = localStorage.getItem('refresh_token');
		const expires = parseInt(localStorage.getItem('expires'));
		if (access_token && refresh_token && expires) {
			return {
				access_token,
				refresh_token,
				expires: new Date(expires),
			}
		} else {
			return null;
		}
	},
	setLogin(login) {
		localStorage.setItem('access_token', login.access_token);
		if (login.refresh_token) {
			localStorage.setItem('refresh_token', login.refresh_token);
		}
		const expires = new Date();
		expires.setSeconds(expires.getSeconds() + login.expires_in);
		localStorage.setItem('expires', expires.getTime());
	}

};

const api = {
	async login_refresh_token(refresh_token) {
		let response = await fetch('/api/refresh-token', {
			method: "POST",
			body: JSON.stringify({ refresh_token })
		});
		return await response.json();
	},
	async login_confirm_code(code) {
		let response = await fetch('/api/confirm_code', {
			method: "POST",
			body: JSON.stringify({ code }),
		});
		return await response.json();
	},
	async subscription_list(token) {
		let response = await fetch('/api/subscription/list', {
			method: "POST",
			body: token
		});
		return await response.json();
	},
}

const Login = {
	template: `<ol>
	<li>Browse to the google login <a href="/api/redirect-login" target="blank">Here</a></li>
	<li>
		Enter your code:<br />
		<form @submit="tryLogin">
		<input v-model="code" />
		<button type="submit">Log in</button>
		</form>
	</li>
</ol>`,
	created() {
		let self = this;
		let login = storage.login();
		if (login) {
			if (login.expires < new Date()) {
				api.login_refresh_token(login.refresh_token).then(r => {
					storage.setLogin(r);
					self.$emit('on_login');
				});
			} else {
				self.$emit('on_login');
			}
		}
	},
	methods: {
		tryLogin(e) {
			e.preventDefault();
			let self = this;
			api.login_confirm_code(this.code).then(r => {
				storage.setLogin(r);
				self.$emit('on_login');
			}).catch(e => {
				console.error(e);
			});
		},
	},
	data() {
		return {
			code: ""
		}
	}
}

const SubscriptionList = {
	template: `<div>
		<div v-for="channel in subscriptions" class="channel">
			<img :src="channel.thumbnail" />
			<span class="title">{{ channel.title }}</span>
		</div>
	</div>`,
	created() {
		let self = this;
		api.subscription_list(storage.login().access_token).then(list => {
			self.subscriptions = list;
		}).catch(e => {
			console.error(e);
		});
	},
	data() {
		return {
			subscriptions: []
		}
	}
}

const App = {
	template: `<div v-if="!logged_in">
	<login @on_login="onLogin" />
</div>
<div v-if="logged_in">
	<subscription-list />
</div>`,
	methods: {
		onLogin() {
			this.logged_in = true;
		}
	},
	data() {
		return {
			logged_in: false
		}
	}
};

Vue.createApp({})
	.component("Login", Login)
	.component("SubscriptionList", SubscriptionList)
	.component("App", App)
	.mount('#main')
