import { storage } from "./connect";
import { ref, uploadBytes, getDownloadURL, deleteObject, listAll, updateMetadata, getMetadata } from "firebase/storage";
import UUID from 'uuidjs';
import imageCompression from "browser-image-compression";

export const update_metadata = async(url: string) => {
    console.log(url);
    const usedRef = ref(storage, url)
    const newMetadata = {
        customMetadata: {
          'used': "true",
        }
    };
    updateMetadata(usedRef, newMetadata).catch((error) => {
        console.error("update_metadata is error", error);
    })
}

const compress_image_file = async(file:File) =>{
    const options = {
        maxSizeMB: 1, 
        maxWidthOrHeight: 800
    };
    try {
        return await imageCompression(file, options);
    } catch (error) {
        console.error("compress_image_file is error", error);
        throw error;
    }
}

export const upload = async(file: File):Promise<string> => {
    
    const ID = UUID.generate();
    const fileType =  file.type.split("/").pop();
    const storageRef = ref(storage, "images/"+ID+"."+fileType);

    let target: File;
    if (fileType !== "mp4") {
        target = await compress_image_file(file);
    } else {
        target = file;
    }

    const metadata = {
        customMetadata: {
          'used': "false",
        }
    };
    return new Promise((resolve) => {
        uploadBytes(storageRef, target, metadata).then((snapshot) => {
            getDownloadURL(snapshot.ref).then((downloadURL) => {
                resolve(downloadURL);
                alert("Upload!");
            }).catch((error) => {
                console.error("upload_file is error", error);
            })
        });
    })
}

export const fetch_unused = async():Promise<Array<string>> => {
    const listRef = ref(storage, "images");
    const unusedList:Array<string> = [];
    return new Promise(async (resolve) => {
        listAll(listRef).then((res) => {

            const responce = Promise.all(res.items.map(async itemRef => {
                await getMetadata(itemRef).then((metadata) => {
                    if (metadata.customMetadata) {
                        if (metadata.customMetadata.used === "false")  {
                            return new Promise(async (resolve) => {
                                await getDownloadURL(itemRef).then((downloadURL) => {
                                    unusedList.push(downloadURL)
                                    resolve(downloadURL);
                                })
                            });
                        }
                    }
                }).catch((error) => {
                    console.error("get_matadata is error", error);
                })
            }));
            responce.then(() => {
                resolve(unusedList);
            })
        }).catch((error) => {
            console.error("fetch_unused is error", error);
        });
    })
}

export const del_from_url = async(url: string):Promise<boolean> => {
    const desertRef = ref(storage, url);
    const result = await new Promise<boolean>((resolve, reject) => {
        deleteObject(desertRef).then(() => {
            console.log("deleted");
            resolve(true);
        }).catch((error) => {
            console.error("del_from_url is error", error);
        });
    })
    return result? true: false;
}