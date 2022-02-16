import { signOut, signInWithEmailAndPassword } from "firebase/auth";
import { auth } from "./connect";

export const sign_in = async(email: string, password: string):Promise<boolean> => {
    let retunrBool = false;
    await signInWithEmailAndPassword(auth, email, password).then((userCredential)=> {
        const user = userCredential.user;
        console.log("sign in");
        retunrBool = true;
    }).catch ((error) => {
        alert("サインインに失敗しました。\nメールアドレスとパスワードを確認の上、再度お試しください。");
        retunrBool = false;
    })
    return retunrBool;
}

export const sign_out = async(email: string):Promise<boolean>  => {
    let retunrBool = false;
    await signOut(auth).then(() => {
        console.log("Sign-out successful.")
        retunrBool = true;
    }).catch((error) => {
        alert("サインアウトに失敗しました。\n再度お試しください。")
        retunrBool = false;
    })

    return retunrBool;
}


const initFirebaseAuth = () => {
    return new Promise((resolve) => {
        var unsubscribe = auth.onAuthStateChanged((user) => {
            resolve(user);
    
            unsubscribe();
        });
    });
};

export const is_signed_in = async(): Promise<boolean> => {
    const user = await initFirebaseAuth();
    return user? true: false;
}