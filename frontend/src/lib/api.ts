import axios, { Axios } from 'axios';

class LocalApi {
  axios!: Axios;

  constructor() {
    this.axios = axios.create({ baseURL: '/api/local' });
  }

  async getPublicKey(): Promise<string> {
    return (await this.axios.get('/public_key')).data as string;
  }

  async getAddr(): Promise<string> {
    return (await this.axios.get('/addr')).data as string;
  }
}

class AccountApi {
  axios!: Axios;

  constructor() {
    this.axios = axios.create({ baseURL: '/api/account' });
  }

  async login(username: string, password: string, fp: string) {
    const payload = { username, password, fp };
    return (await this.axios.post('/login', payload)).data;
  }
}

class Api {
  public local!: LocalApi;
  public account!: AccountApi;

  constructor() {
    this.local = new LocalApi;
    this.account = new AccountApi;
  }

}

export const api = new Api;
