import { Program } from "@coral-xyz/anchor";
import { stakePoolWrapperIdl} from "stake-pool-wrapper-idl";

const stakePoolWrapperProgram = new Program(stakePoolWrapperIdl, provider);
