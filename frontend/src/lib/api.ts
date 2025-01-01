import axios, { Axios } from 'axios';

class AccountApi {
  axios!: Axios;

  constructor() {
    this.axios = axios.create({ baseURL: '/api/account' });
  }

  async getPublicKey(): Promise<string> {
    return (await this.axios.get('/public_key')).data as string;
  }
}

class Api {
  public account!: AccountApi;

  constructor() {
    this.account = new AccountApi;
  }
}

export const api = new Api;
