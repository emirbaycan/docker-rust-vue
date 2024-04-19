import { defineStore } from 'pinia'

export const useUserStore = defineStore('user', {
  state: () => {
    return {
      id: localStorage.id,
      avatar: localStorage.avatar,
      fullname: localStorage.fullname,
      username: localStorage.username,
      email: localStorage.email,
      memberSince: '8/12/2020',
      pfp: localStorage.avatar
    }
  },

  actions: {
    async changeFullname(fullname: string) {
      const response = await fetch(import.meta.env.VITE_API_URL + 'api/user/account', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        credentials: 'include',
        body: JSON.stringify({
           "fullname": fullname 
          }),
      })

      const result = await response.json()

      if(result.status=="success"){
        localStorage.fullname = fullname;
        this.fullname = fullname
      }     
    },
    
    async changeUsername(username: string) {
      const response = await fetch(import.meta.env.VITE_API_URL + 'api/user/account', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        credentials: 'include',
        body: JSON.stringify({
           "username": username 
          }),
      })

      const result = await response.json()

      console.log(result);

      if(result.status=="success"){
        localStorage.username = username;
        this.username = username
      }     
    },
  },
})
