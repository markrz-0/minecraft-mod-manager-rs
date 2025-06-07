import { ModFile } from "./ModFile"

export type FolderContentsEvent = {
  available: ModFile[],
  installed: ModFile[],
}