import { initializeApp } from "firebase/app";
import { getAuth } from "firebase/auth";
import { getFirestore, initializeFirestore } from "firebase/firestore";
import { getStorage } from "firebase/storage";


if(!process.env.FIREBASE_CONFIG){
    throw new Error("No FIREBASE_CONFIG")
}

const firebaseConfig = JSON.parse(process.env.FIREBASE_CONFIG);

const app = initializeApp(firebaseConfig);

initializeFirestore(app, {
	ignoreUndefinedProperties: true,
})

const auth = getAuth(app);
const db = getFirestore(app);
const storage = getStorage(app);

export const get_store = (s: string) => {
    console.log("store")
    console.log(s)
    console.log(db)
    // return db
}
// export const get_auth = () => {
//     return auth
// }
// export const get_storage = () => {
//     return storage
// }