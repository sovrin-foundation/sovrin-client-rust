﻿using Hyperledger.Indy.AnonCredsApi;
using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.DidApi;
using Hyperledger.Indy.WalletApi;
using System;
using Hyperledger.Indy.PaymentsApi;

namespace Hyperledger.Indy
{
    /// <summary>
    /// Exception indicating a problem originating from the Indy SDK.
    /// </summary>
    public class IndyException : Exception
    {
        /// <summary>
        /// Initializes a new IndyException with the specified message and SDK error code.
        /// </summary>
        /// <param name="message">The message for the exception.</param>
        /// <param name="sdkErrorCode">The SDK error code for the exception.</param>
        internal IndyException(String message, int sdkErrorCode) : base(message)
        {
            SdkErrorCode = sdkErrorCode;
        }

        /// <summary>
        /// Generates an IndyException or one of its subclasses from the provided SDK error code.
        /// </summary>
        /// <param name="sdkErrorCode">The error code.</param>
        /// <returns>An IndyException or subclass instance.</returns>
        internal static IndyException FromSdkError(int sdkErrorCode)
        {
            var errorCode = (ErrorCode)sdkErrorCode;
            
            switch (errorCode)
            {
                case ErrorCode.CommonInvalidParam1:
                case ErrorCode.CommonInvalidParam2:
                case ErrorCode.CommonInvalidParam3:
                case ErrorCode.CommonInvalidParam4:
                case ErrorCode.CommonInvalidParam5:
                case ErrorCode.CommonInvalidParam6:
                case ErrorCode.CommonInvalidParam7:
                case ErrorCode.CommonInvalidParam8:
                case ErrorCode.CommonInvalidParam9:
                case ErrorCode.CommonInvalidParam10:
                case ErrorCode.CommonInvalidParam11:
                case ErrorCode.CommonInvalidParam12:
                    return new InvalidParameterException(sdkErrorCode);
                case ErrorCode.CommonInvalidState:
                    return new InvalidStateException();
                case ErrorCode.CommonInvalidStructure:
                    return new InvalidStructureException();
                case ErrorCode.CommonIOError:
                    return new IOException();
                case ErrorCode.WalletInvalidHandle:
                    return new InvalidWalletException(); 
                case ErrorCode.WalletUnknownTypeError:
                    return new UnknownWalletTypeException(); 
                case ErrorCode.WalletTypeAlreadyRegisteredError:
                    return new DuplicateWalletTypeException();
                case ErrorCode.WalletAlreadyExistsError:
                    return new WalletExistsException();
                case ErrorCode.WalletNotFoundError:
                    return new WalletNotFoundException();
                case ErrorCode.WalletIncompatiblePoolError:
                    return new WrongWalletForPoolException();
                case ErrorCode.WalletAlreadyOpenedError:
                    return new WalletAlreadyOpenedException();
                case ErrorCode.WalletAccessFailed:
                    return new WalletAccessFailedException();
                case ErrorCode.PoolLedgerNotCreatedError:
                    return new PoolConfigNotCreatedException();
                case ErrorCode.PoolLedgerInvalidPoolHandle:
                    return new InvalidPoolException();
                case ErrorCode.PoolLedgerTerminated:
                    return new PoolLedgerTerminatedException();
                case ErrorCode.LedgerNoConsensusError:
                    return new LedgerConsensusException();
                case ErrorCode.LedgerInvalidTransaction:
                    return new InvalidLedgerTransactionException();
                case ErrorCode.LedgerSecurityError:
                    return new LedgerSecurityException();
                case ErrorCode.PoolLedgerConfigAlreadyExistsError:
                    return new PoolLedgerConfigExistsException();
                case ErrorCode.AnoncredsRevocationRegistryFullError:
                    return new RevocationRegistryFullException();
                case ErrorCode.AnoncredsInvalidUserRevocIndex:
                    return new InvalidUserRevocIndexException();
                case ErrorCode.AnoncredsMasterSecretDuplicateNameError:
                    return new DuplicateMasterSecretNameException();
                case ErrorCode.AnoncredsProofRejected:
                    return new ProofRejectedException();
                case ErrorCode.AnoncredsCredentialRevoked:
                    return new ClaimRevokedException();
                case ErrorCode.AnoncredsCredDefAlreadyExists:
                    return new CredentialDefinitionAlreadyExistsException();
                case ErrorCode.SignusUnknownCryptoError:
                    return new UnknownCryptoException();
                case ErrorCode.WalletItemNotFoundError:
                    return new WalletItemNotFoundException();
                case ErrorCode.WalletItemAlreadyExistsError:
                    return new WalletItemAlreadyExistsException();
                case ErrorCode.WalletQueryError:
                    return new WalletInvalidQueryException();
                case ErrorCode.WalletStorageError:
                    return new WalletStorageException();
                case ErrorCode.ExtraFundsError:
                    return new ExtraFundsException();
                case ErrorCode.IncompatiblePaymentError:
                    return new IncompatiblePaymentException();
                case ErrorCode.InsufficientFundsError:
                    return new InsufficientFundsException();
                case ErrorCode.PaymentOperationNotSupportedError:
                    return new PaymentOperationNotSupportedException();
                case ErrorCode.PaymentSourceDoesNotExistError:
                    return new PaymentSourceDoesNotExistException();
                case ErrorCode.UnknownPaymentMethod:
                    return new UnknownPaymentMethodException();
                default:
                    var message = $"An unmapped error with the code '{sdkErrorCode}' was returned by the SDK.";
                    return new IndyException(message, sdkErrorCode);
            }      
        }

        /// <summary>
        /// Gets the error code for the exception.
        /// </summary>
        public int SdkErrorCode { get; private set; }
    }

}
