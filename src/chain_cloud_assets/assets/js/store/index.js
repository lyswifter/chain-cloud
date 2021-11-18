

import Vuex from 'vuex'
import Vue from 'vue';
import { Principal } from '@dfinity/principal';
Vue.use(Vuex)
export default new Vuex.Store({
    ICIdentity: {
        principle: null
    },
    CanisterInfo: {
        CommitCanister: null
    },
    getters: {
        getPrinciple: (ICIdentity) => () => {
            if (!ICIdentity.principle) {
                let identity = localStorage.getItem("principleString")
                if (identity) {
                    ICIdentity.principle = identity;
                }
            }
            return ICIdentity.principle
        },
        getCommitCanister: (CanisterInfo) => () => {
            return CanisterInfo.CommitCanister
        }
    },
    mutations: {
        ICIdentityConfig(ICIdentity, principle) {
            ICIdentity.principle = principle
        },
        CleanIdentity(ICIdentity) {
            ICIdentity.principle = null
            localStorage.removeItem('principleString')
        },
        CommitCanisterConfig(CanisterInfo, canister) {
            CanisterInfo.CommitCanister = canister
        }
    },
    actions: {
        setICIdentityConfig({ commit }, principle) {
            if (!principle) {
                return
            }
            localStorage.setItem("principleString",principle )
            commit('ICIdentityConfig', principle)
        },
        removeICIdentity({ commit }) {
            commit('CleanIdentity')
        },
        setCommitCanisterConfig({ commit }, canister) {
            commit('CommitCanisterConfig', canister)
        }
    }
})