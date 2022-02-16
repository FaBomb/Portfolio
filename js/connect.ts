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

export const auth = getAuth(app);
export const store = getFirestore(app);
export const storage = getStorage(app);
