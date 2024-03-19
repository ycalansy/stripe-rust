#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BankConnectionsResourceBalanceCurrent {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutSessionPaymentStatus {
    NoPaymentRequired,
    Paid,
    Unpaid,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionAfterExpiration {
    pub recovery: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardStatus {
    Active,
    Canceled,
    Inactive,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsSofortSetupFutureUsage {
    None,
    OffSession,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TransferSchedule {
    pub delay_days: i64,
    pub interval: String,
    pub monthly_anchor: Option<i64>,
    pub weekly_anchor: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CheckoutPixPaymentMethodOptions {
    pub expires_after_seconds: i64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GelatoDocumentReportType {
    DrivingLicense,
    IdCard,
    Passport,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentProcessingType {
    Card,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Discount {
    pub checkout_session: String,
    pub coupon: Coupon,
    pub customer: ReplaceMeWithAnyOfSpec,
    pub end: i64,
    pub id: String,
    pub invoice: String,
    pub invoice_item: String,
    pub object: String,
    pub promotion_code: ReplaceMeWithAnyOfSpec,
    pub start: i64,
    pub subscription: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem {
    AC,
    AD,
    AE,
    AF,
    AG,
    AI,
    AL,
    AM,
    AO,
    AQ,
    AR,
    AT,
    AU,
    AW,
    AX,
    AZ,
    BA,
    BB,
    BD,
    BE,
    BF,
    BG,
    BH,
    BI,
    BJ,
    BL,
    BM,
    BN,
    BO,
    BQ,
    BR,
    BS,
    BT,
    BV,
    BW,
    BY,
    BZ,
    CA,
    CD,
    CF,
    CG,
    CH,
    CI,
    CK,
    CL,
    CM,
    CN,
    CO,
    CR,
    CV,
    CW,
    CY,
    CZ,
    DE,
    DJ,
    DK,
    DM,
    DO,
    DZ,
    EC,
    EE,
    EG,
    EH,
    ER,
    ES,
    ET,
    FI,
    FJ,
    FK,
    FO,
    FR,
    GA,
    GB,
    GD,
    GE,
    GF,
    GG,
    GH,
    GI,
    GL,
    GM,
    GN,
    GP,
    GQ,
    GR,
    GS,
    GT,
    GU,
    GW,
    GY,
    HK,
    HN,
    HR,
    HT,
    HU,
    ID,
    IE,
    IL,
    IM,
    IN,
    IO,
    IQ,
    IS,
    IT,
    JE,
    JM,
    JO,
    JP,
    KE,
    KG,
    KH,
    KI,
    KM,
    KN,
    KR,
    KW,
    KY,
    KZ,
    LA,
    LB,
    LC,
    LI,
    LK,
    LR,
    LS,
    LT,
    LU,
    LV,
    LY,
    MA,
    MC,
    MD,
    ME,
    MF,
    MG,
    MK,
    ML,
    MM,
    MN,
    MO,
    MQ,
    MR,
    MS,
    MT,
    MU,
    MV,
    MW,
    MX,
    MY,
    MZ,
    NA,
    NC,
    NE,
    NG,
    NI,
    NL,
    NO,
    NP,
    NR,
    NU,
    NZ,
    OM,
    PA,
    PE,
    PF,
    PG,
    PH,
    PK,
    PL,
    PM,
    PN,
    PR,
    PS,
    PT,
    PY,
    QA,
    RE,
    RO,
    RS,
    RU,
    RW,
    SA,
    SB,
    SC,
    SE,
    SG,
    SH,
    SI,
    SJ,
    SK,
    SL,
    SM,
    SN,
    SO,
    SR,
    SS,
    ST,
    SV,
    SX,
    SZ,
    TA,
    TC,
    TD,
    TF,
    TG,
    TH,
    TJ,
    TK,
    TL,
    TM,
    TN,
    TO,
    TR,
    TT,
    TV,
    TW,
    TZ,
    UA,
    UG,
    US,
    UY,
    UZ,
    VA,
    VC,
    VE,
    VG,
    VN,
    VU,
    WF,
    WS,
    XK,
    YE,
    YT,
    ZA,
    ZM,
    ZW,
    ZZ,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodCardWalletApplePay {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OutboundTransfersPaymentMethodDetailsUsBankAccount {
    pub account_holder_type: OutboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType,
    pub account_type: OutboundTransfersPaymentMethodDetailsUsBankAccountAccountType,
    pub bank_name: String,
    pub fingerprint: String,
    pub last4: String,
    pub network: OutboundTransfersPaymentMethodDetailsUsBankAccountNetwork,
    pub routing_number: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AccountAnnualRevenue {
    pub amount: i64,
    pub currency: String,
    pub fiscal_year_end: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesIdealPayments {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodCashapp {
    pub buyer_id: String,
    pub cashtag: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FeeRefund {
    pub amount: i64,
    pub balance_transaction: ReplaceMeWithAnyOfSpec,
    pub created: i64,
    pub currency: String,
    pub fee: ReplaceMeWithAnyOfSpec,
    pub id: String,
    pub metadata: HashMap<String, String>,
    pub object: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalReader {
    pub action: ReplaceMeWithAnyOfSpec,
    pub device_sw_version: String,
    pub device_type: TerminalReaderDeviceType,
    pub id: String,
    pub ip_address: String,
    pub label: String,
    pub livemode: bool,
    pub location: ReplaceMeWithAnyOfSpec,
    pub metadata: HashMap<String, String>,
    pub object: String,
    pub serial_number: String,
    pub status: TerminalReaderStatus,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryFinancialAccountsResourceOutboundTransfers {
    pub ach: Option<TreasuryFinancialAccountsResourceAchToggleSettings>,
    pub us_domestic_wire: Option<TreasuryFinancialAccountsResourceToggleSettings>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingTransactionFlightData {
    pub departure_at: i64,
    pub passenger_name: String,
    pub refundable: bool,
    pub segments: Vec<IssuingTransactionFlightDataLeg>,
    pub travel_agency: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasurySharedResourceBillingDetails {
    pub address: Address,
    pub email: String,
    pub name: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsAuBecsDebit {
    pub bsb_number: String,
    pub fingerprint: String,
    pub last4: String,
    pub mandate: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Dispute {
    pub amount: i64,
    pub balance_transactions: Vec<BalanceTransaction>,
    pub charge: ReplaceMeWithAnyOfSpec,
    pub created: i64,
    pub currency: String,
    pub evidence: DisputeEvidence,
    pub evidence_details: DisputeEvidenceDetails,
    pub id: String,
    pub is_charge_refundable: bool,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub network_reason_code: Option<String>,
    pub object: String,
    pub payment_intent: ReplaceMeWithAnyOfSpec,
    pub payment_method_details: Option<DisputePaymentMethodDetails>,
    pub reason: String,
    pub status: DisputeStatus,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesCashappPayments {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardAuthorizationControlsAllowedCategoriesItem {
    AcRefrigerationRepair,
    AccountingBookkeepingServices,
    AdvertisingServices,
    AgriculturalCooperative,
    AirlinesAirCarriers,
    AirportsFlyingFields,
    AmbulanceServices,
    AmusementParksCarnivals,
    AntiqueReproductions,
    AntiqueShops,
    Aquariums,
    ArchitecturalSurveyingServices,
    ArtDealersAndGalleries,
    ArtistsSupplyAndCraftShops,
    AutoAndHomeSupplyStores,
    AutoBodyRepairShops,
    AutoPaintShops,
    AutoServiceShops,
    AutomatedCashDisburse,
    AutomatedFuelDispensers,
    AutomobileAssociations,
    AutomotivePartsAndAccessoriesStores,
    AutomotiveTireStores,
    BailAndBondPayments,
    Bakeries,
    BandsOrchestras,
    BarberAndBeautyShops,
    BettingCasinoGambling,
    BicycleShops,
    BilliardPoolEstablishments,
    BoatDealers,
    BoatRentalsAndLeases,
    BookStores,
    BooksPeriodicalsAndNewspapers,
    BowlingAlleys,
    BusLines,
    BusinessSecretarialSchools,
    BuyingShoppingServices,
    CableSatelliteAndOtherPayTelevisionAndRadio,
    CameraAndPhotographicSupplyStores,
    CandyNutAndConfectioneryStores,
    CarAndTruckDealersNewUsed,
    CarAndTruckDealersUsedOnly,
    CarRentalAgencies,
    CarWashes,
    CarpentryServices,
    CarpetUpholsteryCleaning,
    Caterers,
    CharitableAndSocialServiceOrganizationsFundraising,
    ChemicalsAndAlliedProducts,
    ChildCareServices,
    ChildrensAndInfantsWearStores,
    ChiropodistsPodiatrists,
    Chiropractors,
    CigarStoresAndStands,
    CivicSocialFraternalAssociations,
    CleaningAndMaintenance,
    ClothingRental,
    CollegesUniversities,
    CommercialEquipment,
    CommercialFootwear,
    CommercialPhotographyArtAndGraphics,
    CommuterTransportAndFerries,
    ComputerNetworkServices,
    ComputerProgramming,
    ComputerRepair,
    ComputerSoftwareStores,
    ComputersPeripheralsAndSoftware,
    ConcreteWorkServices,
    ConstructionMaterials,
    ConsultingPublicRelations,
    CorrespondenceSchools,
    CosmeticStores,
    CounselingServices,
    CountryClubs,
    CourierServices,
    CourtCosts,
    CreditReportingAgencies,
    CruiseLines,
    DairyProductsStores,
    DanceHallStudiosSchools,
    DatingEscortServices,
    DentistsOrthodontists,
    DepartmentStores,
    DetectiveAgencies,
    DigitalGoodsApplications,
    DigitalGoodsGames,
    DigitalGoodsLargeVolume,
    DigitalGoodsMedia,
    DirectMarketingCatalogMerchant,
    DirectMarketingCombinationCatalogAndRetailMerchant,
    DirectMarketingInboundTelemarketing,
    DirectMarketingInsuranceServices,
    DirectMarketingOther,
    DirectMarketingOutboundTelemarketing,
    DirectMarketingSubscription,
    DirectMarketingTravel,
    DiscountStores,
    Doctors,
    DoorToDoorSales,
    DraperyWindowCoveringAndUpholsteryStores,
    DrinkingPlaces,
    DrugStoresAndPharmacies,
    DrugsDrugProprietariesAndDruggistSundries,
    DryCleaners,
    DurableGoods,
    DutyFreeStores,
    EatingPlacesRestaurants,
    EducationalServices,
    ElectricRazorStores,
    ElectricVehicleCharging,
    ElectricalPartsAndEquipment,
    ElectricalServices,
    ElectronicsRepairShops,
    ElectronicsStores,
    ElementarySecondarySchools,
    EmergencyServicesGcasVisaUseOnly,
    EmploymentTempAgencies,
    EquipmentRental,
    ExterminatingServices,
    FamilyClothingStores,
    FastFoodRestaurants,
    FinancialInstitutions,
    FinesGovernmentAdministrativeEntities,
    FireplaceFireplaceScreensAndAccessoriesStores,
    FloorCoveringStores,
    Florists,
    FloristsSuppliesNurseryStockAndFlowers,
    FreezerAndLockerMeatProvisioners,
    FuelDealersNonAutomotive,
    FuneralServicesCrematories,
    FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances,
    FurnitureRepairRefinishing,
    FurriersAndFurShops,
    GeneralServices,
    GiftCardNoveltyAndSouvenirShops,
    GlassPaintAndWallpaperStores,
    GlasswareCrystalStores,
    GolfCoursesPublic,
    GovernmentLicensedHorseDogRacingUsRegionOnly,
    GovernmentLicensedOnlineCasionsOnlineGamblingUsRegionOnly,
    GovernmentOwnedLotteriesNonUsRegion,
    GovernmentOwnedLotteriesUsRegionOnly,
    GovernmentServices,
    GroceryStoresSupermarkets,
    HardwareEquipmentAndSupplies,
    HardwareStores,
    HealthAndBeautySpas,
    HearingAidsSalesAndSupplies,
    HeatingPlumbingAC,
    HobbyToyAndGameShops,
    HomeSupplyWarehouseStores,
    Hospitals,
    HotelsMotelsAndResorts,
    HouseholdApplianceStores,
    IndustrialSupplies,
    InformationRetrievalServices,
    InsuranceDefault,
    InsuranceUnderwritingPremiums,
    IntraCompanyPurchases,
    JewelryStoresWatchesClocksAndSilverwareStores,
    LandscapingServices,
    Laundries,
    LaundryCleaningServices,
    LegalServicesAttorneys,
    LuggageAndLeatherGoodsStores,
    LumberBuildingMaterialsStores,
    ManualCashDisburse,
    MarinasServiceAndSupplies,
    Marketplaces,
    MasonryStoneworkAndPlaster,
    MassageParlors,
    MedicalAndDentalLabs,
    MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies,
    MedicalServices,
    MembershipOrganizations,
    MensAndBoysClothingAndAccessoriesStores,
    MensWomensClothingStores,
    MetalServiceCenters,
    Miscellaneous,
    MiscellaneousApparelAndAccessoryShops,
    MiscellaneousAutoDealers,
    MiscellaneousBusinessServices,
    MiscellaneousFoodStores,
    MiscellaneousGeneralMerchandise,
    MiscellaneousGeneralServices,
    MiscellaneousHomeFurnishingSpecialtyStores,
    MiscellaneousPublishingAndPrinting,
    MiscellaneousRecreationServices,
    MiscellaneousRepairShops,
    MiscellaneousSpecialtyRetail,
    MobileHomeDealers,
    MotionPictureTheaters,
    MotorFreightCarriersAndTrucking,
    MotorHomesDealers,
    MotorVehicleSuppliesAndNewParts,
    MotorcycleShopsAndDealers,
    MotorcycleShopsDealers,
    MusicStoresMusicalInstrumentsPianosAndSheetMusic,
    NewsDealersAndNewsstands,
    NonFiMoneyOrders,
    NonFiStoredValueCardPurchaseLoad,
    NondurableGoods,
    NurseriesLawnAndGardenSupplyStores,
    NursingPersonalCare,
    OfficeAndCommercialFurniture,
    OpticiansEyeglasses,
    OptometristsOphthalmologist,
    OrthopedicGoodsProstheticDevices,
    Osteopaths,
    PackageStoresBeerWineAndLiquor,
    PaintsVarnishesAndSupplies,
    ParkingLotsGarages,
    PassengerRailways,
    PawnShops,
    PetShopsPetFoodAndSupplies,
    PetroleumAndPetroleumProducts,
    PhotoDeveloping,
    PhotographicPhotocopyMicrofilmEquipmentAndSupplies,
    PhotographicStudios,
    PictureVideoProduction,
    PieceGoodsNotionsAndOtherDryGoods,
    PlumbingHeatingEquipmentAndSupplies,
    PoliticalOrganizations,
    PostalServicesGovernmentOnly,
    PreciousStonesAndMetalsWatchesAndJewelry,
    ProfessionalServices,
    PublicWarehousingAndStorage,
    QuickCopyReproAndBlueprint,
    Railroads,
    RealEstateAgentsAndManagersRentals,
    RecordStores,
    RecreationalVehicleRentals,
    ReligiousGoodsStores,
    ReligiousOrganizations,
    RoofingSidingSheetMetal,
    SecretarialSupportServices,
    SecurityBrokersDealers,
    ServiceStations,
    SewingNeedleworkFabricAndPieceGoodsStores,
    ShoeRepairHatCleaning,
    ShoeStores,
    SmallApplianceRepair,
    SnowmobileDealers,
    SpecialTradeServices,
    SpecialtyCleaning,
    SportingGoodsStores,
    SportingRecreationCamps,
    SportsAndRidingApparelStores,
    SportsClubsFields,
    StampAndCoinStores,
    StationaryOfficeSuppliesPrintingAndWritingPaper,
    StationeryStoresOfficeAndSchoolSupplyStores,
    SwimmingPoolsSales,
    TUiTravelGermany,
    TailorsAlterations,
    TaxPaymentsGovernmentAgencies,
    TaxPreparationServices,
    TaxicabsLimousines,
    TelecommunicationEquipmentAndTelephoneSales,
    TelecommunicationServices,
    TelegraphServices,
    TentAndAwningShops,
    TestingLaboratories,
    TheatricalTicketAgencies,
    Timeshares,
    TireRetreadingAndRepair,
    TollsBridgeFees,
    TouristAttractionsAndExhibits,
    TowingServices,
    TrailerParksCampgrounds,
    TransportationServices,
    TravelAgenciesTourOperators,
    TruckStopIteration,
    TruckUtilityTrailerRentals,
    TypesettingPlateMakingAndRelatedServices,
    TypewriterStores,
    USFederalGovernmentAgenciesOrDepartments,
    UniformsCommercialClothing,
    UsedMerchandiseAndSecondhandStores,
    Utilities,
    VarietyStores,
    VeterinaryServices,
    VideoAmusementGameSupplies,
    VideoGameArcades,
    VideoTapeRentalStores,
    VocationalTradeSchools,
    WatchJewelryRepair,
    WeldingRepair,
    WholesaleClubs,
    WigAndToupeeStores,
    WiresMoneyOrders,
    WomensAccessoryAndSpecialtyShops,
    WomensReadyToWearStores,
    WreckingAndSalvageYards,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortalCustomerUpdateAllowedUpdatesItem {
    Address,
    Email,
    Name,
    Phone,
    Shipping,
    TaxId,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ConnectAccountReference {
    pub account: Option<ReplaceMeWithAnyOfSpec>,
    pub r#type: ConnectAccountReferenceType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetupIntentPaymentMethodOptionsCardMandateOptions {
    pub amount: i64,
    pub amount_type: SetupIntentPaymentMethodOptionsCardMandateOptionsAmountType,
    pub currency: String,
    pub description: String,
    pub end_date: i64,
    pub interval: SetupIntentPaymentMethodOptionsCardMandateOptionsInterval,
    pub interval_count: i64,
    pub reference: String,
    pub start_date: i64,
    pub supported_types: Vec<
        SetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypesItem,
    >,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SubscriptionsTrialsResourceEndBehavior {
    pub missing_payment_method: SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingAuthorizationStatus {
    Closed,
    Pending,
    Reversed,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PackageDimensions {
    pub height: f64,
    pub length: f64,
    pub weight: f64,
    pub width: f64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoicesPaymentSettingsPaymentMethodTypesItem {
    AchCreditTransfer,
    AchDebit,
    AcssDebit,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Boleto,
    Card,
    Cashapp,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Konbini,
    Link,
    P24,
    Paynow,
    Paypal,
    Promptpay,
    SepaCreditTransfer,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDomainResourcePaymentMethodStatus {
    pub status: PaymentMethodDomainResourcePaymentMethodStatusStatus,
    pub status_details: Option<PaymentMethodDomainResourcePaymentMethodStatusDetails>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CustomerSession {
    pub client_secret: String,
    pub components: Option<CustomerSessionResourceComponents>,
    pub created: i64,
    pub customer: ReplaceMeWithAnyOfSpec,
    pub expires_at: i64,
    pub livemode: bool,
    pub object: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDomainResourcePaymentMethodStatusDetails {
    pub error_message: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodCustomerBalance {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsLink {
    pub country: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PortalSubscriptionUpdate {
    pub default_allowed_updates: Vec<PortalSubscriptionUpdateDefaultAllowedUpdatesItem>,
    pub enabled: bool,
    pub products: Vec<PortalSubscriptionUpdateProduct>,
    pub proration_behavior: PortalSubscriptionUpdateProrationBehavior,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InvoicePaymentMethodOptionsAcssDebit {
    pub mandate_options: Option<InvoicePaymentMethodOptionsAcssDebitMandateOptions>,
    pub verification_method: Option<
        InvoicePaymentMethodOptionsAcssDebitVerificationMethod,
    >,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodOptionsCardMandateOptions {
    pub amount: i64,
    pub amount_type: PaymentMethodOptionsCardMandateOptionsAmountType,
    pub description: String,
    pub end_date: i64,
    pub interval: PaymentMethodOptionsCardMandateOptionsInterval,
    pub interval_count: i64,
    pub reference: String,
    pub start_date: i64,
    pub supported_types: Vec<PaymentMethodOptionsCardMandateOptionsSupportedTypesItem>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultForItem {
    Invoice,
    Subscription,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    LocalAmusementTax,
    LocalLeaseTax,
    StateCommunicationsTax,
    StateSalesTax,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodConfigResourcePaymentMethodProperties {
    pub available: bool,
    pub display_preference: PaymentMethodConfigResourceDisplayPreference,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxProductResourceLineItemTaxRateDetails {
    pub display_name: String,
    pub percentage_decimal: String,
    pub tax_type: TaxProductResourceLineItemTaxRateDetailsTaxType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThreeDSecureDetailsChargeExemptionIndicator {
    LowRisk,
    None,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct QuotesResourceTotalDetailsResourceBreakdown {
    pub discounts: Vec<LineItemsDiscountAmount>,
    pub taxes: Vec<LineItemsTaxAmount>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub account_country: String,
    pub account_name: String,
    pub account_tax_ids: Vec<ReplaceMeWithAnyOfSpec>,
    pub amount_due: i64,
    pub amount_paid: i64,
    pub amount_remaining: i64,
    pub amount_shipping: i64,
    pub application: ReplaceMeWithAnyOfSpec,
    pub application_fee_amount: i64,
    pub attempt_count: i64,
    pub attempted: bool,
    pub auto_advance: Option<bool>,
    pub automatic_tax: AutomaticTax,
    pub billing_reason: InvoiceBillingReason,
    pub charge: ReplaceMeWithAnyOfSpec,
    pub collection_method: InvoiceCollectionMethod,
    pub created: i64,
    pub currency: String,
    pub custom_fields: Vec<InvoiceSettingCustomField>,
    pub customer: ReplaceMeWithAnyOfSpec,
    pub customer_address: ReplaceMeWithAnyOfSpec,
    pub customer_email: String,
    pub customer_name: String,
    pub customer_phone: String,
    pub customer_shipping: ReplaceMeWithAnyOfSpec,
    pub customer_tax_exempt: InvoiceCustomerTaxExempt,
    pub customer_tax_ids: Option<Vec<InvoicesResourceInvoiceTaxId>>,
    pub default_payment_method: ReplaceMeWithAnyOfSpec,
    pub default_source: ReplaceMeWithAnyOfSpec,
    pub default_tax_rates: Vec<TaxRate>,
    pub description: String,
    pub discount: ReplaceMeWithAnyOfSpec,
    pub discounts: Vec<ReplaceMeWithAnyOfSpec>,
    pub due_date: i64,
    pub effective_at: i64,
    pub ending_balance: i64,
    pub footer: String,
    pub from_invoice: ReplaceMeWithAnyOfSpec,
    pub hosted_invoice_url: Option<String>,
    pub id: Option<String>,
    pub invoice_pdf: Option<String>,
    pub issuer: ConnectAccountReference,
    pub last_finalization_error: ReplaceMeWithAnyOfSpec,
    pub latest_revision: ReplaceMeWithAnyOfSpec,
    pub lines: InvoiceLines,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub next_payment_attempt: i64,
    pub number: String,
    pub object: String,
    pub on_behalf_of: ReplaceMeWithAnyOfSpec,
    pub paid: bool,
    pub paid_out_of_band: bool,
    pub payment_intent: ReplaceMeWithAnyOfSpec,
    pub payment_settings: InvoicesPaymentSettings,
    pub period_end: i64,
    pub period_start: i64,
    pub post_payment_credit_notes_amount: i64,
    pub pre_payment_credit_notes_amount: i64,
    pub quote: ReplaceMeWithAnyOfSpec,
    pub receipt_number: String,
    pub rendering: ReplaceMeWithAnyOfSpec,
    pub rendering_options: ReplaceMeWithAnyOfSpec,
    pub shipping_cost: ReplaceMeWithAnyOfSpec,
    pub shipping_details: ReplaceMeWithAnyOfSpec,
    pub starting_balance: i64,
    pub statement_descriptor: String,
    pub status: InvoiceStatus,
    pub status_transitions: InvoicesStatusTransitions,
    pub subscription: ReplaceMeWithAnyOfSpec,
    pub subscription_details: ReplaceMeWithAnyOfSpec,
    pub subscription_proration_date: Option<i64>,
    pub subtotal: i64,
    pub subtotal_excluding_tax: i64,
    pub tax: i64,
    pub test_clock: ReplaceMeWithAnyOfSpec,
    pub threshold_reason: Option<InvoiceThresholdReason>,
    pub total: i64,
    pub total_discount_amounts: Vec<DiscountsResourceDiscountAmount>,
    pub total_excluding_tax: i64,
    pub total_tax_amounts: Vec<InvoiceTaxAmount>,
    pub transfer_data: ReplaceMeWithAnyOfSpec,
    pub webhooks_delivered_at: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PortalSubscriptionCancellationReason {
    pub enabled: bool,
    pub options: Vec<PortalSubscriptionCancellationReasonOptionsItem>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryOutboundPaymentStatus {
    Canceled,
    Failed,
    Posted,
    Processing,
    Returned,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlows {
    pub received_debit: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodConfigResourceDisplayPreferencePreference {
    None,
    Off,
    On,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryReceivedCreditsResourceSourceFlowsDetails {
    pub credit_reversal: Option<TreasuryCreditReversal>,
    pub outbound_payment: Option<TreasuryOutboundPayment>,
    pub payout: Option<Payout>,
    pub r#type: TreasuryReceivedCreditsResourceSourceFlowsDetailsType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentLinksResourceCustomFieldsLabelType {
    Custom,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MandateStatus {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RefundNextAction {
    pub display_details: ReplaceMeWithAnyOfSpec,
    pub r#type: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesIncrementalAuthorizationIncrementalAuthorizationStatus {
    Available,
    Unavailable,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntentCancellationReason {
    Abandoned,
    Duplicate,
    RequestedByCustomer,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxProductResourceJurisdiction {
    pub country: String,
    pub display_name: String,
    pub level: TaxProductResourceJurisdictionLevel,
    pub state: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThreeDSecureDetailsAuthenticationFlow {
    Challenge,
    Frictionless,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition {
    Auto,
    Hidden,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ApplePayDomain {
    pub created: i64,
    pub domain_name: String,
    pub id: String,
    pub livemode: bool,
    pub object: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingNetworkTokenVisa {
    pub card_reference_id: String,
    pub token_reference_id: String,
    pub token_requestor_id: String,
    pub token_risk_score: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodOptionsAlipay {
    pub setup_future_usage: Option<PaymentMethodOptionsAlipaySetupFutureUsage>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ShippingRateFixedAmount {
    pub amount: i64,
    pub currency: String,
    pub currency_options: Option<ShippingRateFixedAmountCurrencyOptions>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitions {
    pub posted_at: i64,
    pub void_at: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ChargeOutcome {
    pub network_status: String,
    pub reason: String,
    pub risk_level: Option<String>,
    pub risk_score: Option<i64>,
    pub rule: Option<ReplaceMeWithAnyOfSpec>,
    pub seller_message: String,
    pub r#type: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductResourceTaxSettingsDefaultsTaxBehavior {
    Exclusive,
    Inclusive,
    InferredByCurrency,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Mandate {
    pub customer_acceptance: CustomerAcceptance,
    pub id: String,
    pub livemode: bool,
    pub multi_use: Option<MandateMultiUse>,
    pub object: String,
    pub on_behalf_of: Option<String>,
    pub payment_method: ReplaceMeWithAnyOfSpec,
    pub payment_method_details: MandatePaymentMethodDetails,
    pub single_use: Option<MandateSingleUse>,
    pub status: MandateStatus,
    pub r#type: MandateType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutIdealPaymentMethodOptionsSetupFutureUsage {
    None,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodOptionsCardPresent {
    pub request_extended_authorization: bool,
    pub request_incremental_authorization_support: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentLinksResourceConsentCollectionTermsOfService {
    None,
    Required,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentPagesCheckoutSessionConsentTermsOfService {
    Accepted,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardType {
    Physical,
    Virtual,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetchItem {
    Balances,
    Transactions,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentPaymentMethodOptionsCardRequestOvercapture {
    IfAvailable,
    Never,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingNetworkTokenNetworkData {
    pub device: Option<IssuingNetworkTokenDevice>,
    pub mastercard: Option<IssuingNetworkTokenMastercard>,
    pub r#type: IssuingNetworkTokenNetworkDataType,
    pub visa: Option<IssuingNetworkTokenVisa>,
    pub wallet_provider: Option<IssuingNetworkTokenWalletProvider>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodEpsBank {
    ArzteUndApothekerBank,
    AustrianAnadiBankAg,
    BankAustria,
    BankhausCarlSpangler,
    BankhausSchelhammerUndSchatteraAg,
    BawagPskAg,
    BksBankAg,
    BrullKallmusBankAg,
    BtvVierLanderBank,
    CapitalBankGraweGruppeAg,
    DeutscheBankAg,
    Dolomitenbank,
    EasybankAg,
    ErsteBankUndSparkassen,
    HypoAlpeadriabankInternationalAg,
    HypoBankBurgenlandAktiengesellschaft,
    HypoNoeLbFurNiederosterreichUWien,
    HypoOberosterreichSalzburgSteiermark,
    HypoTirolBankAg,
    HypoVorarlbergBankAg,
    MarchfelderBank,
    OberbankAg,
    RaiffeisenBankengruppeOsterreich,
    SchoellerbankAg,
    SpardaBankWien,
    VolksbankGruppe,
    VolkskreditbankAg,
    VrBankBraunau,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodKlarna {
    pub dob: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingDisputeEvidenceReason {
    Canceled,
    Duplicate,
    Fraudulent,
    MerchandiseNotAsDescribed,
    NotReceived,
    Other,
    ServiceNotAsDescribed,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SourceTypeAchDebit {
    pub bank_name: Option<String>,
    pub country: Option<String>,
    pub fingerprint: Option<String>,
    pub last4: Option<String>,
    pub routing_number: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig {
    pub splashscreen: Option<ReplaceMeWithAnyOfSpec>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreditNoteLineItem {
    pub amount: i64,
    pub amount_excluding_tax: i64,
    pub description: String,
    pub discount_amount: i64,
    pub discount_amounts: Vec<DiscountsResourceDiscountAmount>,
    pub id: String,
    pub invoice_line_item: Option<String>,
    pub livemode: bool,
    pub object: String,
    pub quantity: i64,
    pub tax_amounts: Vec<CreditNoteTaxAmount>,
    pub tax_rates: Vec<TaxRate>,
    pub r#type: CreditNoteLineItemType,
    pub unit_amount: i64,
    pub unit_amount_decimal: String,
    pub unit_amount_excluding_tax: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisputePaymentMethodDetailsType {
    Card,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PortalFlowsCouponOffer {
    pub coupon: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsPixSetupFutureUsage {
    None,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetupAttemptPaymentMethodDetailsCardChecks {
    pub address_line1_check: String,
    pub address_postal_code_check: String,
    pub cvc_check: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutKonbiniPaymentMethodOptionsSetupFutureUsage {
    None,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeletedPrice {
    pub deleted: bool,
    pub id: String,
    pub object: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsIdealSetupFutureUsage {
    None,
    OffSession,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InvoicesResourceInvoiceTaxId {
    pub r#type: InvoicesResourceInvoiceTaxIdType,
    pub value: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutLinkPaymentMethodOptionsSetupFutureUsage {
    None,
    OffSession,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Price {
    pub active: bool,
    pub billing_scheme: PriceBillingScheme,
    pub created: i64,
    pub currency: String,
    pub currency_options: Option<PriceCurrencyOptions>,
    pub custom_unit_amount: ReplaceMeWithAnyOfSpec,
    pub id: String,
    pub livemode: bool,
    pub lookup_key: String,
    pub metadata: HashMap<String, String>,
    pub nickname: String,
    pub object: String,
    pub product: ReplaceMeWithAnyOfSpec,
    pub recurring: ReplaceMeWithAnyOfSpec,
    pub tax_behavior: PriceTaxBehavior,
    pub tiers: Option<Vec<PriceTier>>,
    pub tiers_mode: PriceTiersMode,
    pub transform_quantity: ReplaceMeWithAnyOfSpec,
    pub r#type: PriceType,
    pub unit_amount: i64,
    pub unit_amount_decimal: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretServiceResourceScopeType {
    Account,
    User,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaxCalculationLineItemTaxBehavior {
    Exclusive,
    Inclusive,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentLinksResourceCustomFieldsLabel {
    pub custom: String,
    pub r#type: PaymentLinksResourceCustomFieldsLabelType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsStripeAccount {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SubscriptionItemBillingThresholds {
    pub usage_gte: i64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryDebitReversalStatus {
    Failed,
    Processing,
    Succeeded,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Transfer {
    pub amount: i64,
    pub amount_reversed: i64,
    pub balance_transaction: ReplaceMeWithAnyOfSpec,
    pub created: i64,
    pub currency: String,
    pub description: String,
    pub destination: ReplaceMeWithAnyOfSpec,
    pub destination_payment: Option<ReplaceMeWithAnyOfSpec>,
    pub id: String,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub object: String,
    pub reversals: TransferReversals,
    pub reversed: bool,
    pub source_transaction: ReplaceMeWithAnyOfSpec,
    pub source_type: Option<String>,
    pub transfer_group: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsPaynow {
    pub reference: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsCardInstallmentsPlan {
    pub count: i64,
    pub interval: PaymentMethodDetailsCardInstallmentsPlanInterval,
    pub r#type: PaymentMethodDetailsCardInstallmentsPlanType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingAuthorizationVerificationDataExpiryCheck {
    Match,
    Mismatch,
    NotProvided,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryFinancialAccountsResourceOutboundPayments {
    pub ach: Option<TreasuryFinancialAccountsResourceAchToggleSettings>,
    pub us_domestic_wire: Option<TreasuryFinancialAccountsResourceToggleSettings>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionCustomFieldsDropdown {
    pub options: Vec<PaymentPagesCheckoutSessionCustomFieldsOption>,
    pub value: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutSessionRedirectOnCompletion {
    Always,
    IfRequired,
    Never,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodUsBankAccountBlocked {
    pub network_code: PaymentMethodUsBankAccountBlockedNetworkCode,
    pub reason: PaymentMethodUsBankAccountBlockedReason,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RadarRadarOptions {
    pub session: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LineItemsTaxAmountTaxabilityReason {
    CustomerExempt,
    NotCollecting,
    NotSubjectToTax,
    NotSupported,
    PortionProductExempt,
    PortionReducedRated,
    PortionStandardRated,
    ProductExempt,
    ProductExemptHoliday,
    ProportionallyRated,
    ReducedRated,
    ReverseCharge,
    StandardRated,
    TaxableBasisReduced,
    ZeroRated,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingCardholderIndividualDob {
    pub day: i64,
    pub month: i64,
    pub year: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingCardholderUserTermsAcceptance {
    pub date: i64,
    pub ip: String,
    pub user_agent: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RadarValueListItem {
    pub created: i64,
    pub created_by: String,
    pub id: String,
    pub livemode: bool,
    pub object: String,
    pub value: String,
    pub value_list: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodP24Bank {
    AliorBank,
    BankMillennium,
    BankNowyBfgSa,
    BankPekaoSa,
    BankiSpbdzielcze,
    Blik,
    BnpParibas,
    Boz,
    CitiHandlowy,
    CreditAgricole,
    Envelobank,
    EtransferPocztowy24,
    GetinBank,
    Ideabank,
    Ing,
    Inteligo,
    MbankMtransfer,
    NestPrzelew,
    NoblePay,
    PbacZIpko,
    PlusBank,
    SantanderPrzelew24,
    TmobileUsbugiBankowe,
    ToyotaBank,
    VolkswagenBank,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InvoiceSettingQuoteSetting {
    pub days_until_due: i64,
    pub issuer: ConnectAccountReference,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxProductResourceTaxTransactionLineItemResourceReversal {
    pub original_line_item: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CheckoutSessionPaymentMethodOptions {
    pub acss_debit: Option<CheckoutAcssDebitPaymentMethodOptions>,
    pub affirm: Option<CheckoutAffirmPaymentMethodOptions>,
    pub afterpay_clearpay: Option<CheckoutAfterpayClearpayPaymentMethodOptions>,
    pub alipay: Option<CheckoutAlipayPaymentMethodOptions>,
    pub au_becs_debit: Option<CheckoutAuBecsDebitPaymentMethodOptions>,
    pub bacs_debit: Option<CheckoutBacsDebitPaymentMethodOptions>,
    pub bancontact: Option<CheckoutBancontactPaymentMethodOptions>,
    pub boleto: Option<CheckoutBoletoPaymentMethodOptions>,
    pub card: Option<CheckoutCardPaymentMethodOptions>,
    pub cashapp: Option<CheckoutCashappPaymentMethodOptions>,
    pub customer_balance: Option<CheckoutCustomerBalancePaymentMethodOptions>,
    pub eps: Option<CheckoutEpsPaymentMethodOptions>,
    pub fpx: Option<CheckoutFpxPaymentMethodOptions>,
    pub giropay: Option<CheckoutGiropayPaymentMethodOptions>,
    pub grabpay: Option<CheckoutGrabPayPaymentMethodOptions>,
    pub ideal: Option<CheckoutIdealPaymentMethodOptions>,
    pub klarna: Option<CheckoutKlarnaPaymentMethodOptions>,
    pub konbini: Option<CheckoutKonbiniPaymentMethodOptions>,
    pub link: Option<CheckoutLinkPaymentMethodOptions>,
    pub oxxo: Option<CheckoutOxxoPaymentMethodOptions>,
    pub p24: Option<CheckoutP24PaymentMethodOptions>,
    pub paynow: Option<CheckoutPaynowPaymentMethodOptions>,
    pub paypal: Option<CheckoutPaypalPaymentMethodOptions>,
    pub pix: Option<CheckoutPixPaymentMethodOptions>,
    pub revolut_pay: Option<CheckoutRevolutPayPaymentMethodOptions>,
    pub sepa_debit: Option<CheckoutSepaDebitPaymentMethodOptions>,
    pub sofort: Option<CheckoutSofortPaymentMethodOptions>,
    pub swish: Option<CheckoutSwishPaymentMethodOptions>,
    pub us_bank_account: Option<CheckoutUsBankAccountPaymentMethodOptions>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SubscriptionsResourcePaymentMethodOptions {
    pub acss_debit: ReplaceMeWithAnyOfSpec,
    pub bancontact: ReplaceMeWithAnyOfSpec,
    pub card: ReplaceMeWithAnyOfSpec,
    pub customer_balance: ReplaceMeWithAnyOfSpec,
    pub konbini: ReplaceMeWithAnyOfSpec,
    pub us_bank_account: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsCardMandateOptionsSupportedTypesItem {
    India,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuotesResourceRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetupAttemptPaymentMethodDetailsUsBankAccount {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ConnectEmbeddedPaymentsFeatures {
    pub capture_payments: bool,
    pub dispute_management: bool,
    pub refund_management: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RefundDestinationDetails {
    pub affirm: Option<DestinationDetailsUnimplemented>,
    pub afterpay_clearpay: Option<DestinationDetailsUnimplemented>,
    pub alipay: Option<DestinationDetailsUnimplemented>,
    pub au_bank_transfer: Option<DestinationDetailsUnimplemented>,
    pub blik: Option<RefundDestinationDetailsGeneric>,
    pub br_bank_transfer: Option<RefundDestinationDetailsGeneric>,
    pub card: Option<RefundDestinationDetailsCard>,
    pub cashapp: Option<DestinationDetailsUnimplemented>,
    pub customer_cash_balance: Option<DestinationDetailsUnimplemented>,
    pub eps: Option<DestinationDetailsUnimplemented>,
    pub eu_bank_transfer: Option<RefundDestinationDetailsGeneric>,
    pub gb_bank_transfer: Option<RefundDestinationDetailsGeneric>,
    pub giropay: Option<DestinationDetailsUnimplemented>,
    pub grabpay: Option<DestinationDetailsUnimplemented>,
    pub jp_bank_transfer: Option<RefundDestinationDetailsGeneric>,
    pub klarna: Option<DestinationDetailsUnimplemented>,
    pub mx_bank_transfer: Option<RefundDestinationDetailsGeneric>,
    pub p24: Option<RefundDestinationDetailsGeneric>,
    pub paynow: Option<DestinationDetailsUnimplemented>,
    pub paypal: Option<DestinationDetailsUnimplemented>,
    pub pix: Option<DestinationDetailsUnimplemented>,
    pub revolut: Option<DestinationDetailsUnimplemented>,
    pub sofort: Option<DestinationDetailsUnimplemented>,
    pub swish: Option<RefundDestinationDetailsGeneric>,
    pub th_bank_transfer: Option<RefundDestinationDetailsGeneric>,
    pub r#type: String,
    pub us_bank_transfer: Option<RefundDestinationDetailsGeneric>,
    pub wechat_pay: Option<DestinationDetailsUnimplemented>,
    pub zip: Option<DestinationDetailsUnimplemented>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LegalEntityPersonVerification {
    pub additional_document: Option<ReplaceMeWithAnyOfSpec>,
    pub details: Option<String>,
    pub details_code: Option<String>,
    pub document: Option<LegalEntityPersonVerificationDocument>,
    pub status: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SubscriptionBillingThresholds {
    pub amount_gte: i64,
    pub reset_billing_cycle_anchor: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct VerificationSessionRedaction {
    pub status: VerificationSessionRedactionStatus,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CheckoutGiropayPaymentMethodOptions {
    pub setup_future_usage: Option<CheckoutGiropayPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentLinksResourceCompletionBehaviorConfirmationPage {
    pub custom_message: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionCurrencyConversion {
    pub amount_subtotal: i64,
    pub amount_total: i64,
    pub fx_rate: String,
    pub source_currency: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesIndiaInternationalPayments {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransformQuantityRound {
    Down,
    Up,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeletedCard {
    pub currency: Option<String>,
    pub deleted: bool,
    pub id: String,
    pub object: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsUsBankAccountAccountType {
    Checking,
    Savings,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub active: bool,
    pub aggregate_usage: PlanAggregateUsage,
    pub amount: i64,
    pub amount_decimal: String,
    pub billing_scheme: PlanBillingScheme,
    pub created: i64,
    pub currency: String,
    pub id: String,
    pub interval: PlanInterval,
    pub interval_count: i64,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub nickname: String,
    pub object: String,
    pub product: ReplaceMeWithAnyOfSpec,
    pub tiers: Option<Vec<PlanTier>>,
    pub tiers_mode: PlanTiersMode,
    pub transform_usage: ReplaceMeWithAnyOfSpec,
    pub trial_period_days: i64,
    pub usage_type: PlanUsageType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductResourceTaxCalculationShippingCostTaxBehavior {
    Exclusive,
    Inclusive,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryReceivedDebitsResourceLinkedFlows {
    pub debit_reversal: String,
    pub inbound_transfer: String,
    pub issuing_authorization: String,
    pub issuing_transaction: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GelatoVerifiedOutputsIdNumberType {
    BrCpf,
    SgNric,
    UsSsn,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InvoicesFromInvoice {
    pub action: String,
    pub invoice: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercaptureStatus {
    Available,
    Unavailable,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CustomerInvoiceCreditBalance {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PortalPaymentMethodUpdate {
    pub enabled: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CheckoutBoletoPaymentMethodOptions {
    pub expires_after_days: i64,
    pub setup_future_usage: Option<CheckoutBoletoPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutFpxPaymentMethodOptionsSetupFutureUsage {
    None,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CountrySpecSupportedBankAccountCurrencies {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry {
    BE,
    DE,
    ES,
    FR,
    IE,
    NL,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CustomerTaxLocationSource {
    BillingAddress,
    IpAddress,
    PaymentMethod,
    ShippingDestination,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsKonbiniSetupFutureUsage {
    None,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodOptionsCardInstallments {
    pub available_plans: Vec<PaymentMethodDetailsCardInstallmentsPlan>,
    pub enabled: bool,
    pub plan: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CustomerCashBalanceTransaction {
    pub adjusted_for_overdraft: Option<
        CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraft,
    >,
    pub applied_to_payment: Option<
        CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransaction,
    >,
    pub created: i64,
    pub currency: String,
    pub customer: ReplaceMeWithAnyOfSpec,
    pub ending_balance: i64,
    pub funded: Option<
        CustomerBalanceResourceCashBalanceTransactionResourceFundedTransaction,
    >,
    pub id: String,
    pub livemode: bool,
    pub net_amount: i64,
    pub object: String,
    pub refunded_from_payment: Option<
        CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransaction,
    >,
    pub transferred_to_balance: Option<
        CustomerBalanceResourceCashBalanceTransactionResourceTransferredToBalance,
    >,
    pub r#type: CustomerCashBalanceTransactionType,
    pub unapplied_from_payment: Option<
        CustomerBalanceResourceCashBalanceTransactionResourceUnappliedFromPaymentTransaction,
    >,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CreditNoteLineItemType {
    CustomLineItem,
    InvoiceLineItem,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalReaderReaderResourceProcessConfig {
    pub skip_tipping: Option<bool>,
    pub tipping: Option<TerminalReaderReaderResourceTippingConfig>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesExtendedAuthorizationExtendedAuthorizationStatus {
    Disabled,
    Enabled,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentLinkBillingAddressCollection {
    Auto,
    Required,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AccountUnificationAccountController {
    pub is_controller: Option<bool>,
    pub r#type: AccountUnificationAccountControllerType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InvoiceLines {
    pub data: Vec<LineItem>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CustomerTax {
    pub automatic_tax: CustomerTaxAutomaticTax,
    pub ip_address: String,
    pub location: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesKonbiniPayments {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinancialConnectionsAccountSupportedPaymentMethodTypesItem {
    Link,
    UsBankAccount,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardSpendingLimitCategoriesItem {
    AcRefrigerationRepair,
    AccountingBookkeepingServices,
    AdvertisingServices,
    AgriculturalCooperative,
    AirlinesAirCarriers,
    AirportsFlyingFields,
    AmbulanceServices,
    AmusementParksCarnivals,
    AntiqueReproductions,
    AntiqueShops,
    Aquariums,
    ArchitecturalSurveyingServices,
    ArtDealersAndGalleries,
    ArtistsSupplyAndCraftShops,
    AutoAndHomeSupplyStores,
    AutoBodyRepairShops,
    AutoPaintShops,
    AutoServiceShops,
    AutomatedCashDisburse,
    AutomatedFuelDispensers,
    AutomobileAssociations,
    AutomotivePartsAndAccessoriesStores,
    AutomotiveTireStores,
    BailAndBondPayments,
    Bakeries,
    BandsOrchestras,
    BarberAndBeautyShops,
    BettingCasinoGambling,
    BicycleShops,
    BilliardPoolEstablishments,
    BoatDealers,
    BoatRentalsAndLeases,
    BookStores,
    BooksPeriodicalsAndNewspapers,
    BowlingAlleys,
    BusLines,
    BusinessSecretarialSchools,
    BuyingShoppingServices,
    CableSatelliteAndOtherPayTelevisionAndRadio,
    CameraAndPhotographicSupplyStores,
    CandyNutAndConfectioneryStores,
    CarAndTruckDealersNewUsed,
    CarAndTruckDealersUsedOnly,
    CarRentalAgencies,
    CarWashes,
    CarpentryServices,
    CarpetUpholsteryCleaning,
    Caterers,
    CharitableAndSocialServiceOrganizationsFundraising,
    ChemicalsAndAlliedProducts,
    ChildCareServices,
    ChildrensAndInfantsWearStores,
    ChiropodistsPodiatrists,
    Chiropractors,
    CigarStoresAndStands,
    CivicSocialFraternalAssociations,
    CleaningAndMaintenance,
    ClothingRental,
    CollegesUniversities,
    CommercialEquipment,
    CommercialFootwear,
    CommercialPhotographyArtAndGraphics,
    CommuterTransportAndFerries,
    ComputerNetworkServices,
    ComputerProgramming,
    ComputerRepair,
    ComputerSoftwareStores,
    ComputersPeripheralsAndSoftware,
    ConcreteWorkServices,
    ConstructionMaterials,
    ConsultingPublicRelations,
    CorrespondenceSchools,
    CosmeticStores,
    CounselingServices,
    CountryClubs,
    CourierServices,
    CourtCosts,
    CreditReportingAgencies,
    CruiseLines,
    DairyProductsStores,
    DanceHallStudiosSchools,
    DatingEscortServices,
    DentistsOrthodontists,
    DepartmentStores,
    DetectiveAgencies,
    DigitalGoodsApplications,
    DigitalGoodsGames,
    DigitalGoodsLargeVolume,
    DigitalGoodsMedia,
    DirectMarketingCatalogMerchant,
    DirectMarketingCombinationCatalogAndRetailMerchant,
    DirectMarketingInboundTelemarketing,
    DirectMarketingInsuranceServices,
    DirectMarketingOther,
    DirectMarketingOutboundTelemarketing,
    DirectMarketingSubscription,
    DirectMarketingTravel,
    DiscountStores,
    Doctors,
    DoorToDoorSales,
    DraperyWindowCoveringAndUpholsteryStores,
    DrinkingPlaces,
    DrugStoresAndPharmacies,
    DrugsDrugProprietariesAndDruggistSundries,
    DryCleaners,
    DurableGoods,
    DutyFreeStores,
    EatingPlacesRestaurants,
    EducationalServices,
    ElectricRazorStores,
    ElectricVehicleCharging,
    ElectricalPartsAndEquipment,
    ElectricalServices,
    ElectronicsRepairShops,
    ElectronicsStores,
    ElementarySecondarySchools,
    EmergencyServicesGcasVisaUseOnly,
    EmploymentTempAgencies,
    EquipmentRental,
    ExterminatingServices,
    FamilyClothingStores,
    FastFoodRestaurants,
    FinancialInstitutions,
    FinesGovernmentAdministrativeEntities,
    FireplaceFireplaceScreensAndAccessoriesStores,
    FloorCoveringStores,
    Florists,
    FloristsSuppliesNurseryStockAndFlowers,
    FreezerAndLockerMeatProvisioners,
    FuelDealersNonAutomotive,
    FuneralServicesCrematories,
    FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances,
    FurnitureRepairRefinishing,
    FurriersAndFurShops,
    GeneralServices,
    GiftCardNoveltyAndSouvenirShops,
    GlassPaintAndWallpaperStores,
    GlasswareCrystalStores,
    GolfCoursesPublic,
    GovernmentLicensedHorseDogRacingUsRegionOnly,
    GovernmentLicensedOnlineCasionsOnlineGamblingUsRegionOnly,
    GovernmentOwnedLotteriesNonUsRegion,
    GovernmentOwnedLotteriesUsRegionOnly,
    GovernmentServices,
    GroceryStoresSupermarkets,
    HardwareEquipmentAndSupplies,
    HardwareStores,
    HealthAndBeautySpas,
    HearingAidsSalesAndSupplies,
    HeatingPlumbingAC,
    HobbyToyAndGameShops,
    HomeSupplyWarehouseStores,
    Hospitals,
    HotelsMotelsAndResorts,
    HouseholdApplianceStores,
    IndustrialSupplies,
    InformationRetrievalServices,
    InsuranceDefault,
    InsuranceUnderwritingPremiums,
    IntraCompanyPurchases,
    JewelryStoresWatchesClocksAndSilverwareStores,
    LandscapingServices,
    Laundries,
    LaundryCleaningServices,
    LegalServicesAttorneys,
    LuggageAndLeatherGoodsStores,
    LumberBuildingMaterialsStores,
    ManualCashDisburse,
    MarinasServiceAndSupplies,
    Marketplaces,
    MasonryStoneworkAndPlaster,
    MassageParlors,
    MedicalAndDentalLabs,
    MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies,
    MedicalServices,
    MembershipOrganizations,
    MensAndBoysClothingAndAccessoriesStores,
    MensWomensClothingStores,
    MetalServiceCenters,
    Miscellaneous,
    MiscellaneousApparelAndAccessoryShops,
    MiscellaneousAutoDealers,
    MiscellaneousBusinessServices,
    MiscellaneousFoodStores,
    MiscellaneousGeneralMerchandise,
    MiscellaneousGeneralServices,
    MiscellaneousHomeFurnishingSpecialtyStores,
    MiscellaneousPublishingAndPrinting,
    MiscellaneousRecreationServices,
    MiscellaneousRepairShops,
    MiscellaneousSpecialtyRetail,
    MobileHomeDealers,
    MotionPictureTheaters,
    MotorFreightCarriersAndTrucking,
    MotorHomesDealers,
    MotorVehicleSuppliesAndNewParts,
    MotorcycleShopsAndDealers,
    MotorcycleShopsDealers,
    MusicStoresMusicalInstrumentsPianosAndSheetMusic,
    NewsDealersAndNewsstands,
    NonFiMoneyOrders,
    NonFiStoredValueCardPurchaseLoad,
    NondurableGoods,
    NurseriesLawnAndGardenSupplyStores,
    NursingPersonalCare,
    OfficeAndCommercialFurniture,
    OpticiansEyeglasses,
    OptometristsOphthalmologist,
    OrthopedicGoodsProstheticDevices,
    Osteopaths,
    PackageStoresBeerWineAndLiquor,
    PaintsVarnishesAndSupplies,
    ParkingLotsGarages,
    PassengerRailways,
    PawnShops,
    PetShopsPetFoodAndSupplies,
    PetroleumAndPetroleumProducts,
    PhotoDeveloping,
    PhotographicPhotocopyMicrofilmEquipmentAndSupplies,
    PhotographicStudios,
    PictureVideoProduction,
    PieceGoodsNotionsAndOtherDryGoods,
    PlumbingHeatingEquipmentAndSupplies,
    PoliticalOrganizations,
    PostalServicesGovernmentOnly,
    PreciousStonesAndMetalsWatchesAndJewelry,
    ProfessionalServices,
    PublicWarehousingAndStorage,
    QuickCopyReproAndBlueprint,
    Railroads,
    RealEstateAgentsAndManagersRentals,
    RecordStores,
    RecreationalVehicleRentals,
    ReligiousGoodsStores,
    ReligiousOrganizations,
    RoofingSidingSheetMetal,
    SecretarialSupportServices,
    SecurityBrokersDealers,
    ServiceStations,
    SewingNeedleworkFabricAndPieceGoodsStores,
    ShoeRepairHatCleaning,
    ShoeStores,
    SmallApplianceRepair,
    SnowmobileDealers,
    SpecialTradeServices,
    SpecialtyCleaning,
    SportingGoodsStores,
    SportingRecreationCamps,
    SportsAndRidingApparelStores,
    SportsClubsFields,
    StampAndCoinStores,
    StationaryOfficeSuppliesPrintingAndWritingPaper,
    StationeryStoresOfficeAndSchoolSupplyStores,
    SwimmingPoolsSales,
    TUiTravelGermany,
    TailorsAlterations,
    TaxPaymentsGovernmentAgencies,
    TaxPreparationServices,
    TaxicabsLimousines,
    TelecommunicationEquipmentAndTelephoneSales,
    TelecommunicationServices,
    TelegraphServices,
    TentAndAwningShops,
    TestingLaboratories,
    TheatricalTicketAgencies,
    Timeshares,
    TireRetreadingAndRepair,
    TollsBridgeFees,
    TouristAttractionsAndExhibits,
    TowingServices,
    TrailerParksCampgrounds,
    TransportationServices,
    TravelAgenciesTourOperators,
    TruckStopIteration,
    TruckUtilityTrailerRentals,
    TypesettingPlateMakingAndRelatedServices,
    TypewriterStores,
    USFederalGovernmentAgenciesOrDepartments,
    UniformsCommercialClothing,
    UsedMerchandiseAndSecondhandStores,
    Utilities,
    VarietyStores,
    VeterinaryServices,
    VideoAmusementGameSupplies,
    VideoGameArcades,
    VideoTapeRentalStores,
    VocationalTradeSchools,
    WatchJewelryRepair,
    WeldingRepair,
    WholesaleClubs,
    WigAndToupeeStores,
    WiresMoneyOrders,
    WomensAccessoryAndSpecialtyShops,
    WomensReadyToWearStores,
    WreckingAndSalvageYards,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsBoletoSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PortalSubscriptionCancel {
    pub cancellation_reason: PortalSubscriptionCancellationReason,
    pub enabled: bool,
    pub mode: PortalSubscriptionCancelMode,
    pub proration_behavior: PortalSubscriptionCancelProrationBehavior,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingNetworkTokenNetworkDataType {
    Mastercard,
    Visa,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxProductRegistrationsResourceCountryOptionsEuStandard {
    pub place_of_supply_scheme: TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutSessionStatus {
    Complete,
    Expired,
    Open,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GelatoDocumentReportStatus {
    Unverified,
    Verified,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryCreditReversal {
    pub amount: i64,
    pub created: i64,
    pub currency: String,
    pub financial_account: String,
    pub hosted_regulatory_receipt_url: String,
    pub id: String,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub network: TreasuryCreditReversalNetwork,
    pub object: String,
    pub received_credit: String,
    pub status: TreasuryCreditReversalStatus,
    pub status_transitions: TreasuryReceivedCreditsResourceStatusTransitions,
    pub transaction: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CheckoutBacsDebitPaymentMethodOptions {
    pub setup_future_usage: Option<
        CheckoutBacsDebitPaymentMethodOptionsSetupFutureUsage,
    >,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodOptionsBacsDebit {
    pub setup_future_usage: Option<PaymentMethodOptionsBacsDebitSetupFutureUsage>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SubscriptionItem {
    pub billing_thresholds: ReplaceMeWithAnyOfSpec,
    pub created: i64,
    pub id: String,
    pub metadata: HashMap<String, String>,
    pub object: String,
    pub plan: Plan,
    pub price: Price,
    pub quantity: Option<i64>,
    pub subscription: String,
    pub tax_rates: Vec<TaxRate>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThreeDSecureDetailsResultReason {
    Abandoned,
    Bypassed,
    Canceled,
    CardNotEnrolled,
    NetworkNotSupported,
    ProtocolError,
    Rejected,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CouponCurrencyOption {
    pub amount_off: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryOutboundPaymentsResourceReturnedStatus {
    pub code: TreasuryOutboundPaymentsResourceReturnedStatusCode,
    pub transaction: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryReceivedDebitsResourceStatusTransitions {
    pub completed_at: i64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardholderAuthorizationControlsBlockedCategoriesItem {
    AcRefrigerationRepair,
    AccountingBookkeepingServices,
    AdvertisingServices,
    AgriculturalCooperative,
    AirlinesAirCarriers,
    AirportsFlyingFields,
    AmbulanceServices,
    AmusementParksCarnivals,
    AntiqueReproductions,
    AntiqueShops,
    Aquariums,
    ArchitecturalSurveyingServices,
    ArtDealersAndGalleries,
    ArtistsSupplyAndCraftShops,
    AutoAndHomeSupplyStores,
    AutoBodyRepairShops,
    AutoPaintShops,
    AutoServiceShops,
    AutomatedCashDisburse,
    AutomatedFuelDispensers,
    AutomobileAssociations,
    AutomotivePartsAndAccessoriesStores,
    AutomotiveTireStores,
    BailAndBondPayments,
    Bakeries,
    BandsOrchestras,
    BarberAndBeautyShops,
    BettingCasinoGambling,
    BicycleShops,
    BilliardPoolEstablishments,
    BoatDealers,
    BoatRentalsAndLeases,
    BookStores,
    BooksPeriodicalsAndNewspapers,
    BowlingAlleys,
    BusLines,
    BusinessSecretarialSchools,
    BuyingShoppingServices,
    CableSatelliteAndOtherPayTelevisionAndRadio,
    CameraAndPhotographicSupplyStores,
    CandyNutAndConfectioneryStores,
    CarAndTruckDealersNewUsed,
    CarAndTruckDealersUsedOnly,
    CarRentalAgencies,
    CarWashes,
    CarpentryServices,
    CarpetUpholsteryCleaning,
    Caterers,
    CharitableAndSocialServiceOrganizationsFundraising,
    ChemicalsAndAlliedProducts,
    ChildCareServices,
    ChildrensAndInfantsWearStores,
    ChiropodistsPodiatrists,
    Chiropractors,
    CigarStoresAndStands,
    CivicSocialFraternalAssociations,
    CleaningAndMaintenance,
    ClothingRental,
    CollegesUniversities,
    CommercialEquipment,
    CommercialFootwear,
    CommercialPhotographyArtAndGraphics,
    CommuterTransportAndFerries,
    ComputerNetworkServices,
    ComputerProgramming,
    ComputerRepair,
    ComputerSoftwareStores,
    ComputersPeripheralsAndSoftware,
    ConcreteWorkServices,
    ConstructionMaterials,
    ConsultingPublicRelations,
    CorrespondenceSchools,
    CosmeticStores,
    CounselingServices,
    CountryClubs,
    CourierServices,
    CourtCosts,
    CreditReportingAgencies,
    CruiseLines,
    DairyProductsStores,
    DanceHallStudiosSchools,
    DatingEscortServices,
    DentistsOrthodontists,
    DepartmentStores,
    DetectiveAgencies,
    DigitalGoodsApplications,
    DigitalGoodsGames,
    DigitalGoodsLargeVolume,
    DigitalGoodsMedia,
    DirectMarketingCatalogMerchant,
    DirectMarketingCombinationCatalogAndRetailMerchant,
    DirectMarketingInboundTelemarketing,
    DirectMarketingInsuranceServices,
    DirectMarketingOther,
    DirectMarketingOutboundTelemarketing,
    DirectMarketingSubscription,
    DirectMarketingTravel,
    DiscountStores,
    Doctors,
    DoorToDoorSales,
    DraperyWindowCoveringAndUpholsteryStores,
    DrinkingPlaces,
    DrugStoresAndPharmacies,
    DrugsDrugProprietariesAndDruggistSundries,
    DryCleaners,
    DurableGoods,
    DutyFreeStores,
    EatingPlacesRestaurants,
    EducationalServices,
    ElectricRazorStores,
    ElectricVehicleCharging,
    ElectricalPartsAndEquipment,
    ElectricalServices,
    ElectronicsRepairShops,
    ElectronicsStores,
    ElementarySecondarySchools,
    EmergencyServicesGcasVisaUseOnly,
    EmploymentTempAgencies,
    EquipmentRental,
    ExterminatingServices,
    FamilyClothingStores,
    FastFoodRestaurants,
    FinancialInstitutions,
    FinesGovernmentAdministrativeEntities,
    FireplaceFireplaceScreensAndAccessoriesStores,
    FloorCoveringStores,
    Florists,
    FloristsSuppliesNurseryStockAndFlowers,
    FreezerAndLockerMeatProvisioners,
    FuelDealersNonAutomotive,
    FuneralServicesCrematories,
    FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances,
    FurnitureRepairRefinishing,
    FurriersAndFurShops,
    GeneralServices,
    GiftCardNoveltyAndSouvenirShops,
    GlassPaintAndWallpaperStores,
    GlasswareCrystalStores,
    GolfCoursesPublic,
    GovernmentLicensedHorseDogRacingUsRegionOnly,
    GovernmentLicensedOnlineCasionsOnlineGamblingUsRegionOnly,
    GovernmentOwnedLotteriesNonUsRegion,
    GovernmentOwnedLotteriesUsRegionOnly,
    GovernmentServices,
    GroceryStoresSupermarkets,
    HardwareEquipmentAndSupplies,
    HardwareStores,
    HealthAndBeautySpas,
    HearingAidsSalesAndSupplies,
    HeatingPlumbingAC,
    HobbyToyAndGameShops,
    HomeSupplyWarehouseStores,
    Hospitals,
    HotelsMotelsAndResorts,
    HouseholdApplianceStores,
    IndustrialSupplies,
    InformationRetrievalServices,
    InsuranceDefault,
    InsuranceUnderwritingPremiums,
    IntraCompanyPurchases,
    JewelryStoresWatchesClocksAndSilverwareStores,
    LandscapingServices,
    Laundries,
    LaundryCleaningServices,
    LegalServicesAttorneys,
    LuggageAndLeatherGoodsStores,
    LumberBuildingMaterialsStores,
    ManualCashDisburse,
    MarinasServiceAndSupplies,
    Marketplaces,
    MasonryStoneworkAndPlaster,
    MassageParlors,
    MedicalAndDentalLabs,
    MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies,
    MedicalServices,
    MembershipOrganizations,
    MensAndBoysClothingAndAccessoriesStores,
    MensWomensClothingStores,
    MetalServiceCenters,
    Miscellaneous,
    MiscellaneousApparelAndAccessoryShops,
    MiscellaneousAutoDealers,
    MiscellaneousBusinessServices,
    MiscellaneousFoodStores,
    MiscellaneousGeneralMerchandise,
    MiscellaneousGeneralServices,
    MiscellaneousHomeFurnishingSpecialtyStores,
    MiscellaneousPublishingAndPrinting,
    MiscellaneousRecreationServices,
    MiscellaneousRepairShops,
    MiscellaneousSpecialtyRetail,
    MobileHomeDealers,
    MotionPictureTheaters,
    MotorFreightCarriersAndTrucking,
    MotorHomesDealers,
    MotorVehicleSuppliesAndNewParts,
    MotorcycleShopsAndDealers,
    MotorcycleShopsDealers,
    MusicStoresMusicalInstrumentsPianosAndSheetMusic,
    NewsDealersAndNewsstands,
    NonFiMoneyOrders,
    NonFiStoredValueCardPurchaseLoad,
    NondurableGoods,
    NurseriesLawnAndGardenSupplyStores,
    NursingPersonalCare,
    OfficeAndCommercialFurniture,
    OpticiansEyeglasses,
    OptometristsOphthalmologist,
    OrthopedicGoodsProstheticDevices,
    Osteopaths,
    PackageStoresBeerWineAndLiquor,
    PaintsVarnishesAndSupplies,
    ParkingLotsGarages,
    PassengerRailways,
    PawnShops,
    PetShopsPetFoodAndSupplies,
    PetroleumAndPetroleumProducts,
    PhotoDeveloping,
    PhotographicPhotocopyMicrofilmEquipmentAndSupplies,
    PhotographicStudios,
    PictureVideoProduction,
    PieceGoodsNotionsAndOtherDryGoods,
    PlumbingHeatingEquipmentAndSupplies,
    PoliticalOrganizations,
    PostalServicesGovernmentOnly,
    PreciousStonesAndMetalsWatchesAndJewelry,
    ProfessionalServices,
    PublicWarehousingAndStorage,
    QuickCopyReproAndBlueprint,
    Railroads,
    RealEstateAgentsAndManagersRentals,
    RecordStores,
    RecreationalVehicleRentals,
    ReligiousGoodsStores,
    ReligiousOrganizations,
    RoofingSidingSheetMetal,
    SecretarialSupportServices,
    SecurityBrokersDealers,
    ServiceStations,
    SewingNeedleworkFabricAndPieceGoodsStores,
    ShoeRepairHatCleaning,
    ShoeStores,
    SmallApplianceRepair,
    SnowmobileDealers,
    SpecialTradeServices,
    SpecialtyCleaning,
    SportingGoodsStores,
    SportingRecreationCamps,
    SportsAndRidingApparelStores,
    SportsClubsFields,
    StampAndCoinStores,
    StationaryOfficeSuppliesPrintingAndWritingPaper,
    StationeryStoresOfficeAndSchoolSupplyStores,
    SwimmingPoolsSales,
    TUiTravelGermany,
    TailorsAlterations,
    TaxPaymentsGovernmentAgencies,
    TaxPreparationServices,
    TaxicabsLimousines,
    TelecommunicationEquipmentAndTelephoneSales,
    TelecommunicationServices,
    TelegraphServices,
    TentAndAwningShops,
    TestingLaboratories,
    TheatricalTicketAgencies,
    Timeshares,
    TireRetreadingAndRepair,
    TollsBridgeFees,
    TouristAttractionsAndExhibits,
    TowingServices,
    TrailerParksCampgrounds,
    TransportationServices,
    TravelAgenciesTourOperators,
    TruckStopIteration,
    TruckUtilityTrailerRentals,
    TypesettingPlateMakingAndRelatedServices,
    TypewriterStores,
    USFederalGovernmentAgenciesOrDepartments,
    UniformsCommercialClothing,
    UsedMerchandiseAndSecondhandStores,
    Utilities,
    VarietyStores,
    VeterinaryServices,
    VideoAmusementGameSupplies,
    VideoGameArcades,
    VideoTapeRentalStores,
    VocationalTradeSchools,
    WatchJewelryRepair,
    WeldingRepair,
    WholesaleClubs,
    WigAndToupeeStores,
    WiresMoneyOrders,
    WomensAccessoryAndSpecialtyShops,
    WomensReadyToWearStores,
    WreckingAndSalvageYards,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentNextActionKonbiniFamilymart {
    pub confirmation_number: Option<String>,
    pub payment_code: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DisputeEvidence {
    pub access_activity_log: String,
    pub billing_address: String,
    pub cancellation_policy: ReplaceMeWithAnyOfSpec,
    pub cancellation_policy_disclosure: String,
    pub cancellation_rebuttal: String,
    pub customer_communication: ReplaceMeWithAnyOfSpec,
    pub customer_email_address: String,
    pub customer_name: String,
    pub customer_purchase_ip: String,
    pub customer_signature: ReplaceMeWithAnyOfSpec,
    pub duplicate_charge_documentation: ReplaceMeWithAnyOfSpec,
    pub duplicate_charge_explanation: String,
    pub duplicate_charge_id: String,
    pub product_description: String,
    pub receipt: ReplaceMeWithAnyOfSpec,
    pub refund_policy: ReplaceMeWithAnyOfSpec,
    pub refund_policy_disclosure: String,
    pub refund_refusal_explanation: String,
    pub service_date: String,
    pub service_documentation: ReplaceMeWithAnyOfSpec,
    pub shipping_address: String,
    pub shipping_carrier: String,
    pub shipping_date: String,
    pub shipping_documentation: ReplaceMeWithAnyOfSpec,
    pub shipping_tracking_number: String,
    pub uncategorized_file: ReplaceMeWithAnyOfSpec,
    pub uncategorized_text: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingDisputeTreasury {
    pub debit_reversal: String,
    pub received_debit: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodCardWalletLink {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SourceTypeAuBecsDebit {
    pub bsb_number: Option<String>,
    pub fingerprint: Option<String>,
    pub last4: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SourceTypeP24 {
    pub reference: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem {
    Aba,
    Iban,
    Sepa,
    SortCode,
    Spei,
    Swift,
    Zengin,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionCustomFieldsText {
    pub maximum_length: i64,
    pub minimum_length: i64,
    pub value: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DisputePaymentMethodDetailsCard {
    pub brand: String,
    pub network_reason_code: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceUnappliedFromPaymentTransaction {
    pub payment_intent: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingToken {
    pub card: ReplaceMeWithAnyOfSpec,
    pub created: i64,
    pub device_fingerprint: String,
    pub id: String,
    pub last4: Option<String>,
    pub livemode: bool,
    pub network: IssuingTokenNetwork,
    pub network_data: Option<IssuingNetworkTokenNetworkData>,
    pub network_updated_at: i64,
    pub object: String,
    pub status: IssuingTokenStatus,
    pub wallet_provider: Option<IssuingTokenWalletProvider>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CheckoutCardPaymentMethodOptions {
    pub installments: Option<CheckoutCardInstallmentsOptions>,
    pub setup_future_usage: Option<CheckoutCardPaymentMethodOptionsSetupFutureUsage>,
    pub statement_descriptor_suffix_kana: Option<String>,
    pub statement_descriptor_suffix_kanji: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutGrabPayPaymentMethodOptionsSetupFutureUsage {
    None,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AccountInvoicesSettings {
    pub default_account_tax_ids: Vec<ReplaceMeWithAnyOfSpec>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LineItemType {
    Invoiceitem,
    Subscription,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionCustomFieldsLabel {
    pub custom: String,
    pub r#type: PaymentPagesCheckoutSessionCustomFieldsLabelType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EphemeralKey {
    pub created: i64,
    pub expires: i64,
    pub id: String,
    pub livemode: bool,
    pub object: String,
    pub secret: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FileLink {
    pub created: i64,
    pub expired: bool,
    pub expires_at: i64,
    pub file: ReplaceMeWithAnyOfSpec,
    pub id: String,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub object: String,
    pub url: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FinancialConnectionsAccountOwner {
    pub email: String,
    pub id: String,
    pub name: String,
    pub object: String,
    pub ownership: String,
    pub phone: String,
    pub raw_address: String,
    pub refreshed_at: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AccountPaymentsSettings {
    pub statement_descriptor: String,
    pub statement_descriptor_kana: String,
    pub statement_descriptor_kanji: String,
    pub statement_descriptor_prefix_kana: String,
    pub statement_descriptor_prefix_kanji: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DestinationDetailsUnimplemented {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MandateAcssDebitDefaultForItem {
    Invoice,
    Subscription,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ClimateProductCurrentPricesPerMetricTon {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OutboundPaymentsPaymentMethodDetailsFinancialAccount {
    pub id: String,
    pub network: OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentSetupFutureUsage {
    OffSession,
    OnSession,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodEps {
    pub bank: PaymentMethodEpsBank,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutAffirmPaymentMethodOptionsSetupFutureUsage {
    None,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionAfterExpirationRecovery {
    pub allow_promotion_codes: bool,
    pub enabled: bool,
    pub expires_at: i64,
    pub url: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SubscriptionScheduleAddInvoiceItem {
    pub price: ReplaceMeWithAnyOfSpec,
    pub quantity: i64,
    pub tax_rates: Option<Vec<TaxRate>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApiErrorsCode {
    AccountClosed,
    AccountCountryInvalidAddress,
    AccountErrorCountryChangeRequiresAdditionalSteps,
    AccountInformationMismatch,
    AccountInvalid,
    AccountNumberInvalid,
    AcssDebitSessionIncomplete,
    AlipayUpgradeRequired,
    AmountTooLarge,
    AmountTooSmall,
    ApiKeyExpired,
    ApplicationFeesNotAllowed,
    AuthenticationRequired,
    BalanceInsufficient,
    BalanceInvalidParameter,
    BankAccountBadRoutingNumbers,
    BankAccountDeclined,
    BankAccountExists,
    BankAccountRestricted,
    BankAccountUnusable,
    BankAccountUnverified,
    BankAccountVerificationFailed,
    BillingInvalidMandate,
    BitcoinUpgradeRequired,
    CaptureChargeAuthorizationExpired,
    CaptureUnauthorizedPayment,
    CardDeclineRateLimitExceeded,
    CardDeclined,
    CardholderPhoneNumberRequired,
    ChargeAlreadyCaptured,
    ChargeAlreadyRefunded,
    ChargeDisputed,
    ChargeExceedsSourceLimit,
    ChargeExpiredForCapture,
    ChargeInvalidParameter,
    ChargeNotRefundable,
    ClearingCodeUnsupported,
    CountryCodeInvalid,
    CountryUnsupported,
    CouponExpired,
    CustomerMaxPaymentMethods,
    CustomerMaxSubscriptions,
    CustomerTaxLocationInvalid,
    DebitNotAuthorized,
    EmailInvalid,
    ExpiredCard,
    FinancialConnectionsAccountInactive,
    FinancialConnectionsNoSuccessfulTransactionRefresh,
    IdempotencyKeyInUse,
    IncorrectAddress,
    IncorrectCvc,
    IncorrectNumber,
    IncorrectZip,
    InstantPayoutsConfigDisabled,
    InstantPayoutsCurrencyDisabled,
    InstantPayoutsLimitExceeded,
    InstantPayoutsUnsupported,
    InsufficientFunds,
    IntentInvalidState,
    IntentVerificationMethodMissing,
    InvalidCardType,
    InvalidCharacters,
    InvalidChargeAmount,
    InvalidCvc,
    InvalidExpiryMonth,
    InvalidExpiryYear,
    InvalidNumber,
    InvalidSourceUsage,
    InvalidTaxLocation,
    InvoiceNoCustomerLineItems,
    InvoiceNoPaymentMethodTypes,
    InvoiceNoSubscriptionLineItems,
    InvoiceNotEditable,
    InvoiceOnBehalfOfNotEditable,
    InvoicePaymentIntentRequiresAction,
    InvoiceUpcomingNone,
    LivemodeMismatch,
    LockTimeout,
    Missing,
    NoAccount,
    NotAllowedOnStandardAccount,
    OutOfInventory,
    OwnershipDeclarationNotAllowed,
    ParameterInvalidEmpty,
    ParameterInvalidInteger,
    ParameterInvalidStringBlank,
    ParameterInvalidStringEmpty,
    ParameterMissing,
    ParameterUnknown,
    ParametersExclusive,
    PaymentIntentActionRequired,
    PaymentIntentAuthenticationFailure,
    PaymentIntentIncompatiblePaymentMethod,
    PaymentIntentInvalidParameter,
    PaymentIntentKonbiniRejectedConfirmationNumber,
    PaymentIntentMandateInvalid,
    PaymentIntentPaymentAttemptExpired,
    PaymentIntentPaymentAttemptFailed,
    PaymentIntentUnexpectedState,
    PaymentMethodBankAccountAlreadyVerified,
    PaymentMethodBankAccountBlocked,
    PaymentMethodBillingDetailsAddressMissing,
    PaymentMethodConfigurationFailures,
    PaymentMethodCurrencyMismatch,
    PaymentMethodCustomerDecline,
    PaymentMethodInvalidParameter,
    PaymentMethodInvalidParameterTestmode,
    PaymentMethodMicrodepositFailed,
    PaymentMethodMicrodepositVerificationAmountsInvalid,
    PaymentMethodMicrodepositVerificationAmountsMismatch,
    PaymentMethodMicrodepositVerificationAttemptsExceeded,
    PaymentMethodMicrodepositVerificationDescriptorCodeMismatch,
    PaymentMethodMicrodepositVerificationTimeout,
    PaymentMethodNotAvailable,
    PaymentMethodProviderDecline,
    PaymentMethodProviderTimeout,
    PaymentMethodUnactivated,
    PaymentMethodUnexpectedState,
    PaymentMethodUnsupportedType,
    PayoutReconciliationNotReady,
    PayoutsLimitExceeded,
    PayoutsNotAllowed,
    PlatformAccountRequired,
    PlatformApiKeyExpired,
    PostalCodeInvalid,
    ProcessingError,
    ProductInactive,
    ProgressiveOnboardingLimitExceeded,
    RateLimit,
    ReferToCustomer,
    RefundDisputedPayment,
    ResourceAlreadyExists,
    ResourceMissing,
    ReturnIntentAlreadyProcessed,
    RoutingNumberInvalid,
    SecretKeyRequired,
    SepaUnsupportedAccount,
    SetupAttemptFailed,
    SetupIntentAuthenticationFailure,
    SetupIntentInvalidParameter,
    SetupIntentMandateInvalid,
    SetupIntentSetupAttemptExpired,
    SetupIntentUnexpectedState,
    ShippingCalculationFailed,
    SkuInactive,
    StateUnsupported,
    StatusTransitionInvalid,
    StripeTaxInactive,
    TaxIdInvalid,
    TaxesCalculationFailed,
    TerminalLocationCountryUnsupported,
    TerminalReaderBusy,
    TerminalReaderHardwareFault,
    TerminalReaderOffline,
    TerminalReaderTimeout,
    TestmodeChargesOnly,
    TlsVersionUnsupported,
    TokenAlreadyUsed,
    TokenCardNetworkInvalid,
    TokenInUse,
    TransferSourceBalanceParametersMismatch,
    TransfersNotAllowed,
    UrlInvalid,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct File {
    pub created: i64,
    pub expires_at: i64,
    pub filename: String,
    pub id: String,
    pub links: Option<FileLinks>,
    pub object: String,
    pub purpose: FilePurpose,
    pub size: i64,
    pub title: String,
    pub r#type: String,
    pub url: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BankConnectionsResourceBalanceRefresh {
    pub last_attempted_at: i64,
    pub next_refresh_available_at: i64,
    pub status: BankConnectionsResourceBalanceRefreshStatus,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionCustomTextPosition {
    pub message: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GelatoReportDocumentOptions {
    pub allowed_types: Option<Vec<GelatoReportDocumentOptionsAllowedTypesItem>>,
    pub require_id_number: Option<bool>,
    pub require_live_capture: Option<bool>,
    pub require_matching_selfie: Option<bool>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AccountFutureRequirements {
    pub alternatives: Vec<AccountRequirementsAlternative>,
    pub current_deadline: i64,
    pub currently_due: Vec<String>,
    pub disabled_reason: String,
    pub errors: Vec<AccountRequirementsError>,
    pub eventually_due: Vec<String>,
    pub past_due: Vec<String>,
    pub pending_verification: Vec<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutOxxoPaymentMethodOptionsSetupFutureUsage {
    None,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingAuthorizationVerificationDataAddressLine1Check {
    Match,
    Mismatch,
    NotProvided,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsPaypal {
    pub payer_email: String,
    pub payer_id: String,
    pub payer_name: String,
    pub seller_protection: ReplaceMeWithAnyOfSpec,
    pub transaction_id: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RadarValueListListItems {
    pub data: Vec<RadarValueListItem>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetupAttemptPaymentMethodDetailsIdeal {
    pub bank: SetupAttemptPaymentMethodDetailsIdealBank,
    pub bic: SetupAttemptPaymentMethodDetailsIdealBic,
    pub generated_sepa_debit: ReplaceMeWithAnyOfSpec,
    pub generated_sepa_debit_mandate: ReplaceMeWithAnyOfSpec,
    pub iban_last4: String,
    pub verified_name: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutCashappPaymentMethodOptionsSetupFutureUsage {
    None,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OfflineAcceptance {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodOptionsCustomerBalanceEuBankAccount {
    pub country: PaymentMethodOptionsCustomerBalanceEuBankAccountCountry,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxProductResourceTaxBreakdown {
    pub amount: i64,
    pub inclusive: bool,
    pub tax_rate_details: TaxProductResourceTaxRateDetails,
    pub taxability_reason: TaxProductResourceTaxBreakdownTaxabilityReason,
    pub taxable_amount: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryFinancialAccountsResourcePlatformRestrictions {
    pub inbound_flows: TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows,
    pub outbound_flows: TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodLink {
    pub email: String,
    pub persistent_token: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    CreditReversal,
    Other,
    OutboundPayment,
    Payout,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortalFlowsFlowType {
    PaymentMethodUpdate,
    SubscriptionCancel,
    SubscriptionUpdate,
    SubscriptionUpdateConfirm,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentNextActionUseStripeSdk {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShippingRateDeliveryEstimateBoundUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxCalculation {
    pub amount_total: i64,
    pub currency: String,
    pub customer: String,
    pub customer_details: TaxProductResourceCustomerDetails,
    pub expires_at: i64,
    pub id: String,
    pub line_items: TaxCalculationLineItems,
    pub livemode: bool,
    pub object: String,
    pub shipping_cost: ReplaceMeWithAnyOfSpec,
    pub tax_amount_exclusive: i64,
    pub tax_amount_inclusive: i64,
    pub tax_breakdown: Vec<TaxProductResourceTaxBreakdown>,
    pub tax_date: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionCustomFields {
    pub dropdown: Option<PaymentPagesCheckoutSessionCustomFieldsDropdown>,
    pub key: String,
    pub label: PaymentPagesCheckoutSessionCustomFieldsLabel,
    pub numeric: Option<PaymentPagesCheckoutSessionCustomFieldsNumeric>,
    pub optional: bool,
    pub text: Option<PaymentPagesCheckoutSessionCustomFieldsText>,
    pub r#type: PaymentPagesCheckoutSessionCustomFieldsType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThreeDSecureDetailsVersion {
    No_102,
    No_210,
    No_220,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentPaymentMethodOptionsCardRequestMulticapture {
    IfAvailable,
    Never,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
    Active,
    Canceled,
    Incomplete,
    IncompleteExpired,
    PastDue,
    Paused,
    Trialing,
    Unpaid,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceStatus {
    Draft,
    Open,
    Paid,
    Uncollectible,
    Void,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentCancellationReason {
    Abandoned,
    Automatic,
    Duplicate,
    FailedInvoice,
    Fraudulent,
    RequestedByCustomer,
    VoidInvoice,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RadarValueListItemType {
    CardBin,
    CardFingerprint,
    CaseSensitiveString,
    Country,
    CustomerId,
    Email,
    IpAddress,
    SepaDebitFingerprint,
    String,
    UsBankAccountFingerprint,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UsBankAccountNetworksSupportedItem {
    Ach,
    UsDomesticWire,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AccountTreasurySettings {
    pub tos_acceptance: Option<AccountTermsOfService>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ScheduledQueryRun {
    pub created: i64,
    pub data_load_time: i64,
    pub error: Option<SigmaScheduledQueryRunError>,
    pub file: ReplaceMeWithAnyOfSpec,
    pub id: String,
    pub livemode: bool,
    pub object: String,
    pub result_available_until: i64,
    pub sql: String,
    pub status: String,
    pub title: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct QuotesResourceTotalDetails {
    pub amount_discount: i64,
    pub amount_shipping: i64,
    pub amount_tax: i64,
    pub breakdown: Option<QuotesResourceTotalDetailsResourceBreakdown>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodCardPresent {
    pub brand: String,
    pub cardholder_name: String,
    pub country: String,
    pub description: Option<String>,
    pub exp_month: i64,
    pub exp_year: i64,
    pub fingerprint: String,
    pub funding: String,
    pub iin: Option<String>,
    pub issuer: Option<String>,
    pub last4: String,
    pub networks: ReplaceMeWithAnyOfSpec,
    pub read_method: PaymentMethodCardPresentReadMethod,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetupIntentNextActionUseStripeSdk {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisputeStatus {
    Lost,
    NeedsResponse,
    UnderReview,
    WarningClosed,
    WarningNeedsResponse,
    WarningUnderReview,
    Won,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AccountSepaDebitPaymentsSettings {
    pub creditor_id: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodBacsDebit {
    pub fingerprint: String,
    pub last4: String,
    pub sort_code: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RadarEarlyFraudWarning {
    pub actionable: bool,
    pub charge: ReplaceMeWithAnyOfSpec,
    pub created: i64,
    pub fraud_type: String,
    pub id: String,
    pub livemode: bool,
    pub object: String,
    pub payment_intent: Option<ReplaceMeWithAnyOfSpec>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceTaxAmountTaxabilityReason {
    CustomerExempt,
    NotCollecting,
    NotSubjectToTax,
    NotSupported,
    PortionProductExempt,
    PortionReducedRated,
    PortionStandardRated,
    ProductExempt,
    ProductExemptHoliday,
    ProportionallyRated,
    ReducedRated,
    ReverseCharge,
    StandardRated,
    TaxableBasisReduced,
    ZeroRated,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentTypeSpecificPaymentMethodOptionsClient {
    pub capture_method: Option<
        PaymentIntentTypeSpecificPaymentMethodOptionsClientCaptureMethod,
    >,
    pub installments: Option<PaymentFlowsInstallmentOptions>,
    pub require_cvc_recollection: Option<bool>,
    pub setup_future_usage: Option<
        PaymentIntentTypeSpecificPaymentMethodOptionsClientSetupFutureUsage,
    >,
    pub verification_method: Option<
        PaymentIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod,
    >,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentNextActionBoleto {
    pub expires_at: i64,
    pub hosted_voucher_url: String,
    pub number: String,
    pub pdf: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ChargeFraudDetails {
    pub stripe_report: Option<String>,
    pub user_report: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingCardholderAddress {
    pub address: Address,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutCustomerBalancePaymentMethodOptionsSetupFutureUsage {
    None,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem {
    AchCreditTransfer,
    AchDebit,
    AcssDebit,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Boleto,
    Card,
    Cashapp,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Konbini,
    Link,
    P24,
    Paynow,
    Paypal,
    Promptpay,
    SepaCreditTransfer,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesTaxReportingUs1099Misc {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsCardWalletSamsungPay {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxProductResourceTaxSettingsStatusDetailsResourcePending {
    pub missing_fields: Vec<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingAuthorizationVerificationDataAddressPostalCodeCheck {
    Match,
    Mismatch,
    NotProvided,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardReplacementReason {
    Damaged,
    Expired,
    Lost,
    Stolen,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CheckoutSession {
    pub after_expiration: ReplaceMeWithAnyOfSpec,
    pub allow_promotion_codes: bool,
    pub amount_subtotal: i64,
    pub amount_total: i64,
    pub automatic_tax: PaymentPagesCheckoutSessionAutomaticTax,
    pub billing_address_collection: CheckoutSessionBillingAddressCollection,
    pub cancel_url: String,
    pub client_reference_id: String,
    pub client_secret: String,
    pub consent: ReplaceMeWithAnyOfSpec,
    pub consent_collection: ReplaceMeWithAnyOfSpec,
    pub created: i64,
    pub currency: String,
    pub currency_conversion: ReplaceMeWithAnyOfSpec,
    pub custom_fields: Vec<PaymentPagesCheckoutSessionCustomFields>,
    pub custom_text: PaymentPagesCheckoutSessionCustomText,
    pub customer: ReplaceMeWithAnyOfSpec,
    pub customer_creation: CheckoutSessionCustomerCreation,
    pub customer_details: ReplaceMeWithAnyOfSpec,
    pub customer_email: String,
    pub expires_at: i64,
    pub id: String,
    pub invoice: ReplaceMeWithAnyOfSpec,
    pub invoice_creation: ReplaceMeWithAnyOfSpec,
    pub line_items: Option<CheckoutSessionLineItems>,
    pub livemode: bool,
    pub locale: CheckoutSessionLocale,
    pub metadata: HashMap<String, String>,
    pub mode: CheckoutSessionMode,
    pub object: String,
    pub payment_intent: ReplaceMeWithAnyOfSpec,
    pub payment_link: ReplaceMeWithAnyOfSpec,
    pub payment_method_collection: CheckoutSessionPaymentMethodCollection,
    pub payment_method_configuration_details: ReplaceMeWithAnyOfSpec,
    pub payment_method_options: ReplaceMeWithAnyOfSpec,
    pub payment_method_types: Vec<String>,
    pub payment_status: CheckoutSessionPaymentStatus,
    pub phone_number_collection: Option<
        PaymentPagesCheckoutSessionPhoneNumberCollection,
    >,
    pub recovered_from: String,
    pub redirect_on_completion: Option<CheckoutSessionRedirectOnCompletion>,
    pub return_url: Option<String>,
    pub setup_intent: ReplaceMeWithAnyOfSpec,
    pub shipping_address_collection: ReplaceMeWithAnyOfSpec,
    pub shipping_cost: ReplaceMeWithAnyOfSpec,
    pub shipping_details: ReplaceMeWithAnyOfSpec,
    pub shipping_options: Vec<PaymentPagesCheckoutSessionShippingOption>,
    pub status: CheckoutSessionStatus,
    pub submit_type: CheckoutSessionSubmitType,
    pub subscription: ReplaceMeWithAnyOfSpec,
    pub success_url: String,
    pub tax_id_collection: Option<PaymentPagesCheckoutSessionTaxIdCollection>,
    pub total_details: ReplaceMeWithAnyOfSpec,
    pub ui_mode: CheckoutSessionUiMode,
    pub url: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BankConnectionsResourceBalanceApiResourceCashBalanceAvailable {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardholderStatus {
    Active,
    Blocked,
    Inactive,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentLinksResourceInvoiceSettings {
    pub account_tax_ids: Vec<ReplaceMeWithAnyOfSpec>,
    pub custom_fields: Vec<InvoiceSettingCustomField>,
    pub description: String,
    pub footer: String,
    pub issuer: ReplaceMeWithAnyOfSpec,
    pub metadata: HashMap<String, String>,
    pub rendering_options: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsInteracPresentReadMethod {
    ContactEmv,
    ContactlessEmv,
    ContactlessMagstripeMode,
    MagneticStripeFallback,
    MagneticStripeTrack2,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AccountExternalAccounts {
    pub data: Vec<ExternalAccount>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetupIntentPaymentMethodOptions {
    pub acss_debit: Option<SetupIntentPaymentMethodOptionsAcssDebit>,
    pub card: Option<SetupIntentPaymentMethodOptionsCard>,
    pub link: Option<SetupIntentPaymentMethodOptionsLink>,
    pub paypal: Option<SetupIntentPaymentMethodOptionsPaypal>,
    pub sepa_debit: Option<SetupIntentPaymentMethodOptionsSepaDebit>,
    pub us_bank_account: Option<SetupIntentPaymentMethodOptionsUsBankAccount>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingAuthorizationVerificationDataCvcCheck {
    Match,
    Mismatch,
    NotProvided,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InvoicesInvoiceRendering {
    pub amount_tax_display: String,
    pub pdf: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer {
    pub country: InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsCardPresentReceiptAccountType {
    Checking,
    Credit,
    Prepaid,
    Unknown,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GelatoReportIdNumberOptions {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsP24SetupFutureUsage {
    None,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ExchangeRateRates {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodOptionsPaypal {
    pub capture_method: Option<PaymentMethodOptionsPaypalCaptureMethod>,
    pub preferred_locale: String,
    pub reference: String,
    pub setup_future_usage: Option<PaymentMethodOptionsPaypalSetupFutureUsage>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinancialConnectionsSessionPermissionsItem {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingCard {
    pub brand: String,
    pub cancellation_reason: IssuingCardCancellationReason,
    pub cardholder: IssuingCardholder,
    pub created: i64,
    pub currency: String,
    pub cvc: Option<String>,
    pub exp_month: i64,
    pub exp_year: i64,
    pub financial_account: Option<String>,
    pub id: String,
    pub last4: String,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub number: Option<String>,
    pub object: String,
    pub replaced_by: ReplaceMeWithAnyOfSpec,
    pub replacement_for: ReplaceMeWithAnyOfSpec,
    pub replacement_reason: IssuingCardReplacementReason,
    pub shipping: ReplaceMeWithAnyOfSpec,
    pub spending_controls: IssuingCardAuthorizationControls,
    pub status: IssuingCardStatus,
    pub r#type: IssuingCardType,
    pub wallets: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingTransactionFuelData {
    pub r#type: String,
    pub unit: String,
    pub unit_cost_decimal: String,
    pub volume_decimal: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PortalSubscriptionPause {
    pub enabled: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Payout {
    pub amount: i64,
    pub arrival_date: i64,
    pub automatic: bool,
    pub balance_transaction: ReplaceMeWithAnyOfSpec,
    pub created: i64,
    pub currency: String,
    pub description: String,
    pub destination: ReplaceMeWithAnyOfSpec,
    pub failure_balance_transaction: ReplaceMeWithAnyOfSpec,
    pub failure_code: String,
    pub failure_message: String,
    pub id: String,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub method: String,
    pub object: String,
    pub original_payout: ReplaceMeWithAnyOfSpec,
    pub reconciliation_status: PayoutReconciliationStatus,
    pub reversed_by: ReplaceMeWithAnyOfSpec,
    pub source_type: String,
    pub statement_descriptor: String,
    pub status: String,
    pub r#type: PayoutType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxSettings {
    pub defaults: TaxProductResourceTaxSettingsDefaults,
    pub head_office: ReplaceMeWithAnyOfSpec,
    pub livemode: bool,
    pub object: String,
    pub status: TaxSettingsStatus,
    pub status_details: TaxProductResourceTaxSettingsStatusDetails,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxProductRegistrationsResourceCountryOptionsUsLocalLeaseTax {
    pub jurisdiction: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountRequirementsErrorCode {
    InvalidAddressCityStatePostalCode,
    InvalidAddressHighwayContractBox,
    InvalidAddressPrivateMailbox,
    InvalidBusinessProfileName,
    InvalidBusinessProfileNameDenylisted,
    InvalidCompanyNameDenylisted,
    InvalidDobAgeOverMaximum,
    InvalidDobAgeUnder18,
    InvalidDobAgeUnderMinimum,
    InvalidProductDescriptionLength,
    InvalidProductDescriptionUrlMatch,
    InvalidRepresentativeCountry,
    InvalidStatementDescriptorBusinessMismatch,
    InvalidStatementDescriptorDenylisted,
    InvalidStatementDescriptorLength,
    InvalidStatementDescriptorPrefixDenylisted,
    InvalidStatementDescriptorPrefixMismatch,
    InvalidStreetAddress,
    InvalidTaxId,
    InvalidTaxIdFormat,
    InvalidTosAcceptance,
    InvalidUrlDenylisted,
    InvalidUrlFormat,
    InvalidUrlLength,
    InvalidUrlWebPresenceDetected,
    InvalidUrlWebsiteBusinessInformationMismatch,
    InvalidUrlWebsiteEmpty,
    InvalidUrlWebsiteInaccessible,
    InvalidUrlWebsiteInaccessibleGeoblocked,
    InvalidUrlWebsiteInaccessiblePasswordProtected,
    InvalidUrlWebsiteIncomplete,
    InvalidUrlWebsiteIncompleteCancellationPolicy,
    InvalidUrlWebsiteIncompleteCustomerServiceDetails,
    InvalidUrlWebsiteIncompleteLegalRestrictions,
    InvalidUrlWebsiteIncompleteRefundPolicy,
    InvalidUrlWebsiteIncompleteReturnPolicy,
    InvalidUrlWebsiteIncompleteTermsAndConditions,
    InvalidUrlWebsiteIncompleteUnderConstruction,
    InvalidUrlWebsiteOther,
    InvalidValueOther,
    VerificationDirectorsMismatch,
    VerificationDocumentAddressMismatch,
    VerificationDocumentAddressMissing,
    VerificationDocumentCorrupt,
    VerificationDocumentCountryNotSupported,
    VerificationDocumentDirectorsMismatch,
    VerificationDocumentDobMismatch,
    VerificationDocumentDuplicateType,
    VerificationDocumentExpired,
    VerificationDocumentFailedCopy,
    VerificationDocumentFailedGreyscale,
    VerificationDocumentFailedOther,
    VerificationDocumentFailedTestMode,
    VerificationDocumentFraudulent,
    VerificationDocumentIdNumberMismatch,
    VerificationDocumentIdNumberMissing,
    VerificationDocumentIncomplete,
    VerificationDocumentInvalid,
    VerificationDocumentIssueOrExpiryDateMissing,
    VerificationDocumentManipulated,
    VerificationDocumentMissingBack,
    VerificationDocumentMissingFront,
    VerificationDocumentNameMismatch,
    VerificationDocumentNameMissing,
    VerificationDocumentNationalityMismatch,
    VerificationDocumentNotReadable,
    VerificationDocumentNotSigned,
    VerificationDocumentNotUploaded,
    VerificationDocumentPhotoMismatch,
    VerificationDocumentTooLarge,
    VerificationDocumentTypeNotSupported,
    VerificationExtraneousDirectors,
    VerificationFailedAddressMatch,
    VerificationFailedBusinessIecNumber,
    VerificationFailedDocumentMatch,
    VerificationFailedIdNumberMatch,
    VerificationFailedKeyedIdentity,
    VerificationFailedKeyedMatch,
    VerificationFailedNameMatch,
    VerificationFailedOther,
    VerificationFailedResidentialAddress,
    VerificationFailedTaxIdMatch,
    VerificationFailedTaxIdNotIssued,
    VerificationMissingDirectors,
    VerificationMissingExecutives,
    VerificationMissingOwners,
    VerificationRequiresAdditionalMemorandumOfAssociations,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesSwishPayments {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoicePaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodOptionsAffirm {
    pub capture_method: Option<PaymentMethodOptionsAffirmCaptureMethod>,
    pub preferred_locale: Option<String>,
    pub setup_future_usage: Option<PaymentMethodOptionsAffirmSetupFutureUsage>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CustomUnitAmount {
    pub maximum: i64,
    pub minimum: i64,
    pub preset: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentNextActionKonbini {
    pub expires_at: i64,
    pub hosted_voucher_url: String,
    pub stores: PaymentIntentNextActionKonbiniStores,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaxIdType {
    AdNrt,
    AeTrn,
    ArCuit,
    AuAbn,
    AuArn,
    BgUic,
    BoTin,
    BrCnpj,
    BrCpf,
    CaBn,
    CaGstHst,
    CaPstBc,
    CaPstMb,
    CaPstSk,
    CaQst,
    ChVat,
    ClTin,
    CnTin,
    CoNit,
    CrTin,
    DoRcn,
    EcRuc,
    EgTin,
    EsCif,
    EuOssVat,
    EuVat,
    GbVat,
    GeVat,
    HkBr,
    HuTin,
    IdNpwp,
    IlVat,
    InGst,
    IsVat,
    JpCn,
    JpRn,
    JpTrn,
    KePin,
    KrBrn,
    LiUid,
    MxRfc,
    MyFrp,
    MyItn,
    MySst,
    NoVat,
    NzGst,
    PeRuc,
    PhTin,
    RoTin,
    RsPib,
    RuInn,
    RuKpp,
    SaVat,
    SgGst,
    SgUen,
    SiTin,
    SvNit,
    ThVat,
    TrTin,
    TwVat,
    UaVat,
    Unknown,
    UsEin,
    UyRuc,
    VeRif,
    VnTin,
    ZaVat,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThreeDSecureDetailsChargeAuthenticationFlow {
    Challenge,
    Frictionless,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransformUsageRound {
    Down,
    Up,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryOutboundTransferStatus {
    Canceled,
    Failed,
    Posted,
    Processing,
    Returned,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BalanceDetail {
    pub available: Vec<BalanceAmount>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentPaymentMethodOptionsSepaDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ConnectEmbeddedBaseFeatures {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CustomerTaxLocation {
    pub country: String,
    pub source: CustomerTaxLocationSource,
    pub state: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodWechatPay {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MandateBacsDebitRevocationReason {
    AccountClosed,
    BankAccountRestricted,
    BankOwnershipChanged,
    CouldNotProcess,
    DebitNotAuthorized,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetupIntentPaymentMethodOptionsUsBankAccount {
    pub financial_connections: Option<LinkedAccountOptionsUsBankAccount>,
    pub mandate_options: Option<PaymentMethodOptionsUsBankAccountMandateOptions>,
    pub verification_method: Option<
        SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod,
    >,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SubscriptionSchedulesResourceDefaultSettingsAutomaticTax {
    pub enabled: bool,
    pub liability: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BankConnectionsResourceTransactionRefresh {
    pub id: String,
    pub last_attempted_at: i64,
    pub next_refresh_available_at: i64,
    pub status: BankConnectionsResourceTransactionRefreshStatus,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod {
    Off,
    OnSubscription,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductResourceTaxRateDetailsTaxType {
    AmusementTax,
    CommunicationsTax,
    Gst,
    Hst,
    Igst,
    Jct,
    LeaseTax,
    Pst,
    Qst,
    Rst,
    SalesTax,
    Vat,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ThreeDSecureDetailsCharge {
    pub authentication_flow: ThreeDSecureDetailsChargeAuthenticationFlow,
    pub electronic_commerce_indicator: ThreeDSecureDetailsChargeElectronicCommerceIndicator,
    pub exemption_indicator: ThreeDSecureDetailsChargeExemptionIndicator,
    pub exemption_indicator_applied: Option<bool>,
    pub result: ThreeDSecureDetailsChargeResult,
    pub result_reason: ThreeDSecureDetailsChargeResultReason,
    pub transaction_id: String,
    pub version: ThreeDSecureDetailsChargeVersion,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Token {
    pub bank_account: Option<BankAccount>,
    pub card: Option<Card>,
    pub client_ip: String,
    pub created: i64,
    pub id: String,
    pub livemode: bool,
    pub object: String,
    pub r#type: String,
    pub used: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryCreditReversalNetwork {
    Ach,
    Stripe,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryTransactionsResourceBalanceImpact {
    pub cash: i64,
    pub inbound_pending: i64,
    pub outbound_pending: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionCustomFieldsNumeric {
    pub maximum_length: i64,
    pub minimum_length: i64,
    pub value: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalConfiguration {
    pub bbpos_wisepos_e: Option<
        TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig,
    >,
    pub id: String,
    pub is_account_default: bool,
    pub livemode: bool,
    pub object: String,
    pub offline: Option<TerminalConfigurationConfigurationResourceOfflineConfig>,
    pub tipping: Option<TerminalConfigurationConfigurationResourceTipping>,
    pub verifone_p400: Option<
        TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig,
    >,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsFpxBank {
    AffinBank,
    Agrobank,
    AllianceBank,
    Ambank,
    BankIslam,
    BankMuamalat,
    BankOfChina,
    BankRakyat,
    Bsn,
    Cimb,
    DeutscheBank,
    HongLeongBank,
    Hsbc,
    Kfh,
    Maybank2e,
    Maybank2u,
    Ocbc,
    PbEnterprise,
    PublicBank,
    Rhb,
    StandardChartered,
    Uob,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsPaypalSetupFutureUsage {
    None,
    OffSession,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentPagesCheckoutSessionCustomFieldsLabelType {
    Custom,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalReaderReaderResourceSetReaderDisplayAction {
    pub cart: ReplaceMeWithAnyOfSpec,
    pub r#type: TerminalReaderReaderResourceSetReaderDisplayActionType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalReaderReaderResourceTippingConfig {
    pub amount_eligible: Option<i64>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsBancontact {
    pub bank_code: String,
    pub bank_name: String,
    pub bic: String,
    pub generated_sepa_debit: ReplaceMeWithAnyOfSpec,
    pub generated_sepa_debit_mandate: ReplaceMeWithAnyOfSpec,
    pub iban_last4: String,
    pub preferred_language: PaymentMethodDetailsBancontactPreferredLanguage,
    pub verified_name: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    OffSession,
    OnSession,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ApplicationFeeRefunds {
    pub data: Vec<FeeRefund>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsCardWalletType {
    AmexExpressCheckout,
    ApplePay,
    GooglePay,
    Link,
    Masterpass,
    SamsungPay,
    VisaCheckout,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Product {
    pub active: bool,
    pub created: i64,
    pub default_price: Option<ReplaceMeWithAnyOfSpec>,
    pub description: String,
    pub features: Vec<ProductFeature>,
    pub id: String,
    pub images: Vec<String>,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub name: String,
    pub object: String,
    pub package_dimensions: ReplaceMeWithAnyOfSpec,
    pub shippable: bool,
    pub statement_descriptor: Option<String>,
    pub tax_code: ReplaceMeWithAnyOfSpec,
    pub r#type: ProductType,
    pub unit_label: Option<String>,
    pub updated: i64,
    pub url: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingDisputeOtherEvidence {
    pub additional_documentation: ReplaceMeWithAnyOfSpec,
    pub explanation: String,
    pub product_description: String,
    pub product_type: IssuingDisputeOtherEvidenceProductType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ThreeDSecureUsage {
    pub supported: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsKonbiniStoreChain {
    Familymart,
    Lawson,
    Ministop,
    Seicomart,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BalanceTransactionType {
    Adjustment,
    Advance,
    AdvanceFunding,
    AnticipationRepayment,
    ApplicationFee,
    ApplicationFeeRefund,
    Charge,
    ClimateOrderPurchase,
    ClimateOrderRefund,
    ConnectCollectionTransfer,
    Contribution,
    IssuingAuthorizationHold,
    IssuingAuthorizationRelease,
    IssuingDispute,
    IssuingTransaction,
    ObligationOutbound,
    ObligationReversalInbound,
    Payment,
    PaymentFailureRefund,
    PaymentNetworkReserveHold,
    PaymentNetworkReserveRelease,
    PaymentRefund,
    PaymentReversal,
    PaymentUnreconciled,
    Payout,
    PayoutCancel,
    PayoutFailure,
    Refund,
    RefundFailure,
    ReserveTransaction,
    ReservedFunds,
    StripeFee,
    StripeFxFee,
    TaxFee,
    Topup,
    TopupReversal,
    Transfer,
    TransferCancel,
    TransferFailure,
    TransferRefund,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentPaymentMethodOptionsCard {
    pub capture_method: Option<PaymentIntentPaymentMethodOptionsCardCaptureMethod>,
    pub installments: ReplaceMeWithAnyOfSpec,
    pub mandate_options: ReplaceMeWithAnyOfSpec,
    pub network: PaymentIntentPaymentMethodOptionsCardNetwork,
    pub request_extended_authorization: Option<
        PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization,
    >,
    pub request_incremental_authorization: Option<
        PaymentIntentPaymentMethodOptionsCardRequestIncrementalAuthorization,
    >,
    pub request_multicapture: Option<
        PaymentIntentPaymentMethodOptionsCardRequestMulticapture,
    >,
    pub request_overcapture: Option<
        PaymentIntentPaymentMethodOptionsCardRequestOvercapture,
    >,
    pub request_three_d_secure: PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure,
    pub require_cvc_recollection: Option<bool>,
    pub setup_future_usage: Option<
        PaymentIntentPaymentMethodOptionsCardSetupFutureUsage,
    >,
    pub statement_descriptor_suffix_kana: Option<String>,
    pub statement_descriptor_suffix_kanji: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentPagesCheckoutSessionTaxIdType {
    AdNrt,
    AeTrn,
    ArCuit,
    AuAbn,
    AuArn,
    BgUic,
    BoTin,
    BrCnpj,
    BrCpf,
    CaBn,
    CaGstHst,
    CaPstBc,
    CaPstMb,
    CaPstSk,
    CaQst,
    ChVat,
    ClTin,
    CnTin,
    CoNit,
    CrTin,
    DoRcn,
    EcRuc,
    EgTin,
    EsCif,
    EuOssVat,
    EuVat,
    GbVat,
    GeVat,
    HkBr,
    HuTin,
    IdNpwp,
    IlVat,
    InGst,
    IsVat,
    JpCn,
    JpRn,
    JpTrn,
    KePin,
    KrBrn,
    LiUid,
    MxRfc,
    MyFrp,
    MyItn,
    MySst,
    NoVat,
    NzGst,
    PeRuc,
    PhTin,
    RoTin,
    RsPib,
    RuInn,
    RuKpp,
    SaVat,
    SgGst,
    SgUen,
    SiTin,
    SvNit,
    ThVat,
    TrTin,
    TwVat,
    UaVat,
    Unknown,
    UsEin,
    UyRuc,
    VeRif,
    VnTin,
    ZaVat,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InvoicesResourceLineItemsProrationDetails {
    pub credited_items: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsBacsDebit {
    pub fingerprint: String,
    pub last4: String,
    pub mandate: String,
    pub sort_code: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxIdVerification {
    pub status: TaxIdVerificationStatus,
    pub verified_address: String,
    pub verified_name: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeletedCustomer {
    pub deleted: bool,
    pub id: String,
    pub object: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod {
    Automatic,
    Instant,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentLinksResourceCompletedSessions {
    pub count: i64,
    pub limit: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LegalEntityCompanyVerificationDocument {
    pub back: ReplaceMeWithAnyOfSpec,
    pub details: String,
    pub details_code: String,
    pub front: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryFinancialAccountsResourceBalanceOutboundPending {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutPaypalPaymentMethodOptionsCaptureMethod {
    Manual,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntentFlowDirectionsItem {
    Inbound,
    Outbound,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransaction {
    pub bank_transfer: CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FileLinks {
    pub data: Vec<FileLink>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Invoiceitem {
    pub amount: i64,
    pub currency: String,
    pub customer: ReplaceMeWithAnyOfSpec,
    pub date: i64,
    pub description: String,
    pub discountable: bool,
    pub discounts: Vec<ReplaceMeWithAnyOfSpec>,
    pub id: String,
    pub invoice: ReplaceMeWithAnyOfSpec,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub object: String,
    pub period: InvoiceLineItemPeriod,
    pub plan: ReplaceMeWithAnyOfSpec,
    pub price: ReplaceMeWithAnyOfSpec,
    pub proration: bool,
    pub quantity: i64,
    pub subscription: ReplaceMeWithAnyOfSpec,
    pub subscription_item: Option<String>,
    pub tax_rates: Vec<TaxRate>,
    pub test_clock: ReplaceMeWithAnyOfSpec,
    pub unit_amount: i64,
    pub unit_amount_decimal: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentLinkLineItems {
    pub data: Vec<Item>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentLinksResourceAfterCompletionType {
    HostedConfirmation,
    Redirect,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsCashappCaptureMethod {
    Manual,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InvoiceSettingSubscriptionScheduleSetting {
    pub account_tax_ids: Option<Vec<ReplaceMeWithAnyOfSpec>>,
    pub days_until_due: i64,
    pub issuer: ConnectAccountReference,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PortalSubscriptionUpdateProduct {
    pub prices: Vec<String>,
    pub product: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct QuotesResourceUpfront {
    pub amount_subtotal: i64,
    pub amount_total: i64,
    pub line_items: Option<QuotesResourceUpfrontLineItems>,
    pub total_details: QuotesResourceTotalDetails,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FundingInstructionsBankTransferZenginRecord {
    pub account_holder_name: String,
    pub account_number: String,
    pub account_type: String,
    pub bank_code: String,
    pub bank_name: String,
    pub branch_code: String,
    pub branch_name: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingAuthorizationTreasury {
    pub received_credits: Vec<String>,
    pub received_debits: Vec<String>,
    pub transaction: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PriceBillingScheme {
    PerUnit,
    Tiered,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingNetworkTokenAddress {
    pub line1: String,
    pub postal_code: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PromotionCodeCurrencyOption {
    pub minimum_amount: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SourceMandateNotificationSepaDebitData {
    pub creditor_identifier: Option<String>,
    pub last4: Option<String>,
    pub mandate_reference: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeletedAccount {
    pub deleted: bool,
    pub id: String,
    pub object: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetupIntentNextActionVerifyWithMicrodeposits {
    pub arrival_date: i64,
    pub hosted_verification_url: String,
    pub microdeposit_type: SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SourceTypeSepaCreditTransfer {
    pub bank_name: Option<String>,
    pub bic: Option<String>,
    pub iban: Option<String>,
    pub refund_account_holder_address_city: Option<String>,
    pub refund_account_holder_address_country: Option<String>,
    pub refund_account_holder_address_line1: Option<String>,
    pub refund_account_holder_address_line2: Option<String>,
    pub refund_account_holder_address_postal_code: Option<String>,
    pub refund_account_holder_address_state: Option<String>,
    pub refund_account_holder_name: Option<String>,
    pub refund_iban: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryReceivedCreditNetwork {
    Ach,
    Card,
    Stripe,
    UsDomesticWire,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesJcbPayments {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsSofort {
    pub bank_code: String,
    pub bank_name: String,
    pub bic: String,
    pub country: String,
    pub generated_sepa_debit: ReplaceMeWithAnyOfSpec,
    pub generated_sepa_debit_mandate: ReplaceMeWithAnyOfSpec,
    pub iban_last4: String,
    pub preferred_language: PaymentMethodDetailsSofortPreferredLanguage,
    pub verified_name: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub account: ReplaceMeWithAnyOfSpec,
    pub future_requirements: Option<AccountCapabilityFutureRequirements>,
    pub id: String,
    pub object: String,
    pub requested: bool,
    pub requested_at: i64,
    pub requirements: Option<AccountCapabilityRequirements>,
    pub status: CapabilityStatus,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsZip {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodOptionsFpx {
    pub setup_future_usage: Option<PaymentMethodOptionsFpxSetupFutureUsage>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsIdealBank {
    AbnAmro,
    AsnBank,
    Bunq,
    Handelsbanken,
    Ing,
    Knab,
    Moneyou,
    N26,
    Nn,
    Rabobank,
    Regiobank,
    Revolut,
    SnsBank,
    TriodosBank,
    VanLanschot,
    Yoursafe,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PromotionCodesResourceRestrictions {
    pub currency_options: Option<PromotionCodesResourceRestrictionsCurrencyOptions>,
    pub first_time_transaction: bool,
    pub minimum_amount: i64,
    pub minimum_amount_currency: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Recurring {
    pub aggregate_usage: RecurringAggregateUsage,
    pub interval: RecurringInterval,
    pub interval_count: i64,
    pub trial_period_days: i64,
    pub usage_type: RecurringUsageType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesEpsPayments {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CheckoutSessionLineItems {
    pub data: Vec<Item>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CheckoutSofortPaymentMethodOptions {
    pub setup_future_usage: Option<CheckoutSofortPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentityVerificationReportType {
    Document,
    IdNumber,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IdentityVerificationSession {
    pub client_secret: String,
    pub created: i64,
    pub id: String,
    pub last_error: ReplaceMeWithAnyOfSpec,
    pub last_verification_report: ReplaceMeWithAnyOfSpec,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub object: String,
    pub options: ReplaceMeWithAnyOfSpec,
    pub redaction: ReplaceMeWithAnyOfSpec,
    pub status: IdentityVerificationSessionStatus,
    pub r#type: IdentityVerificationSessionType,
    pub url: String,
    pub verified_outputs: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingAuthorizationAuthorizationMethod {
    Chip,
    Contactless,
    KeyedIn,
    Online,
    Swipe,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingAuthorizationRequestReason {
    AccountDisabled,
    CardActive,
    CardInactive,
    CardholderInactive,
    CardholderVerificationRequired,
    InsufficientFunds,
    NotAllowed,
    SpendingControls,
    SuspectedFraud,
    VerificationFailed,
    WebhookApproved,
    WebhookDeclined,
    WebhookError,
    WebhookTimeout,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsAffirmCaptureMethod {
    Manual,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaypalSellerProtectionStatus {
    Eligible,
    NotEligible,
    PartiallyEligible,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PlatformTaxFee {
    pub account: String,
    pub id: String,
    pub object: String,
    pub source_transaction: String,
    pub r#type: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionConsent {
    pub promotions: PaymentPagesCheckoutSessionConsentPromotions,
    pub terms_of_service: PaymentPagesCheckoutSessionConsentTermsOfService,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingCardholderVerification {
    pub document: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BankConnectionsResourceBalanceType {
    Cash,
    Credit,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PersonPoliticalExposure {
    Existing,
    None,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GelatoDataDocumentReportExpirationDate {
    pub day: i64,
    pub month: i64,
    pub year: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ProductFeature {
    pub name: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PortalInvoiceList {
    pub enabled: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ChargeTransferData {
    pub amount: i64,
    pub destination: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CreditNoteStatus {
    Issued,
    Void,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutAcssDebitMandateOptionsPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InvoicesShippingCost {
    pub amount_subtotal: i64,
    pub amount_tax: i64,
    pub amount_total: i64,
    pub shipping_rate: ReplaceMeWithAnyOfSpec,
    pub taxes: Option<Vec<LineItemsTaxAmount>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SourceTransactionSepaCreditTransferData {
    pub reference: Option<String>,
    pub sender_iban: Option<String>,
    pub sender_name: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingAuthorizationAuthenticationExemptionClaimedBy {
    Acquirer,
    Issuer,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LegalEntityPersonVerificationDocument {
    pub back: ReplaceMeWithAnyOfSpec,
    pub details: String,
    pub details_code: String,
    pub front: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupAttemptPaymentMethodDetailsSofortPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaxRateJurisdictionLevel {
    City,
    Country,
    County,
    District,
    Multiple,
    State,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CheckoutPaypalPaymentMethodOptions {
    pub capture_method: Option<CheckoutPaypalPaymentMethodOptionsCaptureMethod>,
    pub preferred_locale: String,
    pub reference: String,
    pub setup_future_usage: Option<CheckoutPaypalPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    Company,
    Individual,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaxTransactionLineItemTaxBehavior {
    Exclusive,
    Inclusive,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CustomerBalanceCustomerBalanceSettingsReconciliationMode {
    Automatic,
    Manual,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutCardPaymentMethodOptionsSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GelatoDocumentReportErrorCode {
    DocumentExpired,
    DocumentTypeNotSupported,
    DocumentUnverifiedOther,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentCaptureMethod {
    Automatic,
    AutomaticAsync,
    Manual,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsGiropaySetupFutureUsage {
    None,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutSessionUiMode {
    Embedded,
    Hosted,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AccountDeclineChargeOn {
    pub avs_failure: bool,
    pub cvc_failure: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PersonRequirements {
    pub alternatives: Vec<AccountRequirementsAlternative>,
    pub currently_due: Vec<String>,
    pub errors: Vec<AccountRequirementsError>,
    pub eventually_due: Vec<String>,
    pub past_due: Vec<String>,
    pub pending_verification: Vec<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentNextActionPromptpayDisplayQrCode {
    pub data: String,
    pub hosted_instructions_url: String,
    pub image_url_png: String,
    pub image_url_svg: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsAlipaySetupFutureUsage {
    None,
    OffSession,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChargeStatus {
    Failed,
    Pending,
    Succeeded,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CheckoutCustomerBalanceBankTransferPaymentMethodOptions {
    pub eu_bank_transfer: Option<PaymentMethodOptionsCustomerBalanceEuBankAccount>,
    pub requested_address_types: Option<
        Vec<
            CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypesItem,
        >,
    >,
    pub r#type: CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreditNoteLines {
    pub data: Vec<CreditNoteLineItem>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutAcssDebitMandateOptionsTransactionType {
    Business,
    Personal,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceJpBankTransfer {
    pub sender_bank: String,
    pub sender_branch: String,
    pub sender_name: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxProductResourceTaxSettingsStatusDetailsResourceActive {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows {
    Restricted,
    Unrestricted,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionConsentCollection {
    pub payment_method_reuse_agreement: ReplaceMeWithAnyOfSpec,
    pub promotions: PaymentPagesCheckoutSessionConsentCollectionPromotions,
    pub terms_of_service: PaymentPagesCheckoutSessionConsentCollectionTermsOfService,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PersonAdditionalTosAcceptance {
    pub date: i64,
    pub ip: String,
    pub user_agent: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationSessionRedactionStatus {
    Processing,
    Redacted,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreditNote {
    pub amount: i64,
    pub amount_shipping: i64,
    pub created: i64,
    pub currency: String,
    pub customer: ReplaceMeWithAnyOfSpec,
    pub customer_balance_transaction: ReplaceMeWithAnyOfSpec,
    pub discount_amount: i64,
    pub discount_amounts: Vec<DiscountsResourceDiscountAmount>,
    pub effective_at: i64,
    pub id: String,
    pub invoice: ReplaceMeWithAnyOfSpec,
    pub lines: CreditNoteLines,
    pub livemode: bool,
    pub memo: String,
    pub metadata: HashMap<String, String>,
    pub number: String,
    pub object: String,
    pub out_of_band_amount: i64,
    pub pdf: String,
    pub reason: CreditNoteReason,
    pub refund: ReplaceMeWithAnyOfSpec,
    pub shipping_cost: ReplaceMeWithAnyOfSpec,
    pub status: CreditNoteStatus,
    pub subtotal: i64,
    pub subtotal_excluding_tax: i64,
    pub tax_amounts: Vec<CreditNoteTaxAmount>,
    pub total: i64,
    pub total_excluding_tax: i64,
    pub r#type: CreditNoteType,
    pub voided_at: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodPaynow {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    Exempt,
    None,
    Reverse,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupAttemptFlowDirectionsItem {
    Inbound,
    Outbound,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxProductResourceTaxSettingsStatusDetails {
    pub active: Option<TaxProductResourceTaxSettingsStatusDetailsResourceActive>,
    pub pending: Option<TaxProductResourceTaxSettingsStatusDetailsResourcePending>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalReaderReaderResourceProcessPaymentIntentAction {
    pub payment_intent: ReplaceMeWithAnyOfSpec,
    pub process_config: Option<TerminalReaderReaderResourceProcessConfig>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountsResourceFinancialAddressType {
    Aba,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardGooglePayIneligibleReason {
    MissingAgreement,
    MissingCardholderContact,
    UnsupportedRegion,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Fee {
    pub amount: i64,
    pub application: String,
    pub currency: String,
    pub description: String,
    pub r#type: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentLinksResourceAutomaticTax {
    pub enabled: bool,
    pub liability: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalReaderDeviceType {
    BbposChipper2x,
    BbposWisepad3,
    BbposWiseposE,
    SimulatedWiseposE,
    StripeM2,
    VerifoneP400,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetupIntentPaymentMethodOptionsMandateOptionsAcssDebit {
    pub custom_mandate_url: Option<String>,
    pub default_for: Option<
        Vec<SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultForItem>,
    >,
    pub interval_description: String,
    pub payment_schedule: SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule,
    pub transaction_type: SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SubscriptionAutomaticTax {
    pub enabled: bool,
    pub liability: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
    Challenge,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecurringInterval {
    Day,
    Month,
    Week,
    Year,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaxRegistrationStatus {
    Active,
    Expired,
    Scheduled,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductResourceTaxBreakdownTaxabilityReason {
    CustomerExempt,
    NotCollecting,
    NotSubjectToTax,
    NotSupported,
    PortionProductExempt,
    PortionReducedRated,
    PortionStandardRated,
    ProductExempt,
    ProductExemptHoliday,
    ProportionallyRated,
    ReducedRated,
    ReverseCharge,
    StandardRated,
    TaxableBasisReduced,
    ZeroRated,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentPaymentMethodOptionsAcssDebit {
    pub mandate_options: Option<
        PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebit,
    >,
    pub setup_future_usage: Option<
        PaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage,
    >,
    pub verification_method: Option<
        PaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod,
    >,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsCardWallet {
    pub amex_express_checkout: Option<PaymentMethodDetailsCardWalletAmexExpressCheckout>,
    pub apple_pay: Option<PaymentMethodDetailsCardWalletApplePay>,
    pub dynamic_last4: String,
    pub google_pay: Option<PaymentMethodDetailsCardWalletGooglePay>,
    pub link: Option<PaymentMethodDetailsCardWalletLink>,
    pub masterpass: Option<PaymentMethodDetailsCardWalletMasterpass>,
    pub samsung_pay: Option<PaymentMethodDetailsCardWalletSamsungPay>,
    pub r#type: PaymentMethodDetailsCardWalletType,
    pub visa_checkout: Option<PaymentMethodDetailsCardWalletVisaCheckout>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodCardPresentReadMethod {
    ContactEmv,
    ContactlessEmv,
    ContactlessMagstripeMode,
    MagneticStripeFallback,
    MagneticStripeTrack2,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentPagesCheckoutSessionAutomaticTaxStatus {
    Complete,
    Failed,
    RequiresLocationInputs,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesAuBecsDebitPayments {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CustomerSessionResourceComponentsResourceBuyButton {
    pub enabled: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionShippingAddressCollection {
    pub allowed_countries: Vec<
        PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem,
    >,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GelatoDocumentReport {
    pub address: ReplaceMeWithAnyOfSpec,
    pub dob: ReplaceMeWithAnyOfSpec,
    pub error: ReplaceMeWithAnyOfSpec,
    pub expiration_date: ReplaceMeWithAnyOfSpec,
    pub files: Vec<String>,
    pub first_name: String,
    pub issued_date: ReplaceMeWithAnyOfSpec,
    pub issuing_country: String,
    pub last_name: String,
    pub number: String,
    pub status: GelatoDocumentReportStatus,
    pub r#type: GelatoDocumentReportType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsKlarnaSetupFutureUsage {
    None,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortalFlowsFlowAfterCompletionType {
    HostedConfirmation,
    PortalHomepage,
    Redirect,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PortalFlowsSubscriptionUpdateConfirmItem {
    pub id: String,
    pub price: String,
    pub quantity: Option<i64>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsAchCreditTransfer {
    pub account_number: String,
    pub bank_name: String,
    pub routing_number: String,
    pub swift_code: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodUsBankAccountAccountHolderType {
    Company,
    Individual,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SourceTransaction {
    pub ach_credit_transfer: Option<SourceTransactionAchCreditTransferData>,
    pub amount: i64,
    pub chf_credit_transfer: Option<SourceTransactionChfCreditTransferData>,
    pub created: i64,
    pub currency: String,
    pub gbp_credit_transfer: Option<SourceTransactionGbpCreditTransferData>,
    pub id: String,
    pub livemode: bool,
    pub object: String,
    pub paper_check: Option<SourceTransactionPaperCheckData>,
    pub sepa_credit_transfer: Option<SourceTransactionSepaCreditTransferData>,
    pub source: String,
    pub status: String,
    pub r#type: SourceTransactionType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ConnectCollectionTransfer {
    pub amount: i64,
    pub currency: String,
    pub destination: ReplaceMeWithAnyOfSpec,
    pub id: String,
    pub livemode: bool,
    pub object: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceMandateOptionsCardAmountType {
    Fixed,
    Maximum,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeletedInvoiceitem {
    pub deleted: bool,
    pub id: String,
    pub object: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionSchedulePhaseConfigurationBillingCycleAnchor {
    Automatic,
    PhaseStart,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DisputeEvidenceDetails {
    pub due_by: i64,
    pub has_evidence: bool,
    pub past_due: bool,
    pub submission_count: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodCard {
    pub brand: String,
    pub checks: ReplaceMeWithAnyOfSpec,
    pub country: String,
    pub description: Option<String>,
    pub exp_month: i64,
    pub exp_year: i64,
    pub fingerprint: Option<String>,
    pub funding: String,
    pub iin: Option<String>,
    pub issuer: Option<String>,
    pub last4: String,
    pub networks: ReplaceMeWithAnyOfSpec,
    pub three_d_secure_usage: ReplaceMeWithAnyOfSpec,
    pub wallet: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SourceTypeThreeDSecure {
    pub address_line1_check: Option<String>,
    pub address_zip_check: Option<String>,
    pub authenticated: Option<bool>,
    pub brand: Option<String>,
    pub card: Option<String>,
    pub country: Option<String>,
    pub customer: Option<String>,
    pub cvc_check: Option<String>,
    pub description: Option<String>,
    pub dynamic_last4: Option<String>,
    pub exp_month: Option<i64>,
    pub exp_year: Option<i64>,
    pub fingerprint: Option<String>,
    pub funding: Option<String>,
    pub iin: Option<String>,
    pub issuer: Option<String>,
    pub last4: Option<String>,
    pub name: Option<String>,
    pub three_d_secure: Option<String>,
    pub tokenization_method: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalReaderReaderResourceRefundPaymentAction {
    pub amount: Option<i64>,
    pub charge: Option<ReplaceMeWithAnyOfSpec>,
    pub metadata: HashMap<String, String>,
    pub payment_intent: Option<ReplaceMeWithAnyOfSpec>,
    pub reason: Option<TerminalReaderReaderResourceRefundPaymentActionReason>,
    pub refund: Option<ReplaceMeWithAnyOfSpec>,
    pub refund_application_fee: Option<bool>,
    pub reverse_transfer: Option<bool>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BankAccount {
    pub account: Option<ReplaceMeWithAnyOfSpec>,
    pub account_holder_name: String,
    pub account_holder_type: String,
    pub account_type: String,
    pub available_payout_methods: Option<Vec<BankAccountAvailablePayoutMethodsItem>>,
    pub bank_name: String,
    pub country: String,
    pub currency: String,
    pub customer: Option<ReplaceMeWithAnyOfSpec>,
    pub default_for_currency: Option<bool>,
    pub fingerprint: String,
    pub future_requirements: Option<ReplaceMeWithAnyOfSpec>,
    pub id: String,
    pub last4: String,
    pub metadata: HashMap<String, String>,
    pub object: String,
    pub requirements: Option<ReplaceMeWithAnyOfSpec>,
    pub routing_number: String,
    pub status: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingTokenNetwork {
    Mastercard,
    Visa,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SubscriptionTransferData {
    pub amount_percent: f64,
    pub destination: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FundingInstructionsBankTransferFinancialAddressType {
    Aba,
    Iban,
    SortCode,
    Spei,
    Swift,
    Zengin,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentNextActionCashappQrCode {
    pub expires_at: i64,
    pub image_url_png: String,
    pub image_url_svg: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason {
    AlreadyReversed,
    DeadlinePassed,
    NetworkRestricted,
    Other,
    SourceFlowRestricted,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CheckoutUsBankAccountPaymentMethodOptions {
    pub financial_connections: Option<LinkedAccountOptionsUsBankAccount>,
    pub setup_future_usage: Option<
        CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage,
    >,
    pub verification_method: Option<
        CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod,
    >,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingDisputeServiceNotAsDescribedEvidence {
    pub additional_documentation: ReplaceMeWithAnyOfSpec,
    pub canceled_at: i64,
    pub cancellation_reason: String,
    pub explanation: String,
    pub received_at: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PersonRelationship {
    pub director: bool,
    pub executive: bool,
    pub legal_guardian: bool,
    pub owner: bool,
    pub percent_ownership: f64,
    pub representative: bool,
    pub title: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SourceTypeWechat {
    pub prepay_id: Option<String>,
    pub qr_code_url: Option<String>,
    pub statement_descriptor: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    Ach,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InvoiceThresholdReason {
    pub amount_gte: i64,
    pub item_reasons: Vec<InvoiceItemThresholdReason>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CheckoutAcssDebitMandateOptions {
    pub custom_mandate_url: Option<String>,
    pub default_for: Option<Vec<CheckoutAcssDebitMandateOptionsDefaultForItem>>,
    pub interval_description: String,
    pub payment_schedule: CheckoutAcssDebitMandateOptionsPaymentSchedule,
    pub transaction_type: CheckoutAcssDebitMandateOptionsTransactionType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PromotionCode {
    pub active: bool,
    pub code: String,
    pub coupon: Coupon,
    pub created: i64,
    pub customer: ReplaceMeWithAnyOfSpec,
    pub expires_at: i64,
    pub id: String,
    pub livemode: bool,
    pub max_redemptions: i64,
    pub metadata: HashMap<String, String>,
    pub object: String,
    pub restrictions: PromotionCodesResourceRestrictions,
    pub times_redeemed: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CustomerAcceptance {
    pub accepted_at: i64,
    pub offline: Option<OfflineAcceptance>,
    pub online: Option<OnlineAcceptance>,
    pub r#type: CustomerAcceptanceType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeletedProduct {
    pub deleted: bool,
    pub id: String,
    pub object: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentLinksResourceCustomTextPosition {
    pub message: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsCardPresentReadMethod {
    ContactEmv,
    ContactlessEmv,
    ContactlessMagstripeMode,
    MagneticStripeFallback,
    MagneticStripeTrack2,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryOutboundTransfersResourceReturnedDetails {
    pub code: TreasuryOutboundTransfersResourceReturnedDetailsCode,
    pub transaction: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentProcessingCustomerNotification {
    pub approval_requested: bool,
    pub completes_at: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PortalFlowsAfterCompletionHostedConfirmation {
    pub custom_message: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsZipSetupFutureUsage {
    None,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AccountBusinessProfile {
    pub annual_revenue: ReplaceMeWithAnyOfSpec,
    pub estimated_worker_count: i64,
    pub mcc: String,
    pub monthly_estimated_revenue: Option<AccountMonthlyEstimatedRevenue>,
    pub name: String,
    pub product_description: Option<String>,
    pub support_address: ReplaceMeWithAnyOfSpec,
    pub support_email: String,
    pub support_phone: String,
    pub support_url: String,
    pub url: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentLinkPaymentMethodCollection {
    Always,
    IfRequired,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MandateAcssDebit {
    pub default_for: Option<Vec<MandateAcssDebitDefaultForItem>>,
    pub interval_description: String,
    pub payment_schedule: MandateAcssDebitPaymentSchedule,
    pub transaction_type: MandateAcssDebitTransactionType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MandateSepaDebit {
    pub reference: String,
    pub url: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodSepaDebit {
    pub bank_code: String,
    pub branch_code: String,
    pub country: String,
    pub fingerprint: String,
    pub generated_from: ReplaceMeWithAnyOfSpec,
    pub last4: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaxSettingsStatus {
    Active,
    Pending,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BankConnectionsResourceLinkAccountSessionFilters {
    pub countries: Vec<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TestHelpersTestClock {
    pub created: i64,
    pub deletes_after: i64,
    pub frozen_time: i64,
    pub id: String,
    pub livemode: bool,
    pub name: String,
    pub object: String,
    pub status: TestHelpersTestClockStatus,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsAcssDebit {
    pub bank_name: String,
    pub fingerprint: String,
    pub institution_number: String,
    pub last4: String,
    pub mandate: Option<String>,
    pub transit_number: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesKlarnaPayments {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentLinksResourceCustomFieldsType {
    Dropdown,
    Numeric,
    Text,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RadarReviewResourceSession {
    pub browser: String,
    pub device: String,
    pub platform: String,
    pub version: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RefundNextActionDisplayDetails {
    pub email_sent: EmailSent,
    pub expires_at: i64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryTransactionsResourceFlowDetailsType {
    CreditReversal,
    DebitReversal,
    InboundTransfer,
    IssuingAuthorization,
    Other,
    OutboundPayment,
    OutboundTransfer,
    ReceivedCredit,
    ReceivedDebit,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutPaypalPaymentMethodOptionsSetupFutureUsage {
    None,
    OffSession,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ClimateRemovalsOrderDeliveries {
    pub delivered_at: i64,
    pub location: ReplaceMeWithAnyOfSpec,
    pub metric_tons: String,
    pub registry_url: String,
    pub supplier: ClimateSupplier,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Event {
    pub account: Option<String>,
    pub api_version: String,
    pub created: i64,
    pub data: NotificationEventData,
    pub id: String,
    pub livemode: bool,
    pub object: String,
    pub pending_webhooks: i64,
    pub request: ReplaceMeWithAnyOfSpec,
    pub r#type: EventType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InvoiceInstallmentsCard {
    pub enabled: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Charge {
    pub amount: i64,
    pub amount_captured: i64,
    pub amount_refunded: i64,
    pub application: ReplaceMeWithAnyOfSpec,
    pub application_fee: ReplaceMeWithAnyOfSpec,
    pub application_fee_amount: i64,
    pub authorization_code: Option<String>,
    pub balance_transaction: ReplaceMeWithAnyOfSpec,
    pub billing_details: BillingDetails,
    pub calculated_statement_descriptor: String,
    pub captured: bool,
    pub created: i64,
    pub currency: String,
    pub customer: ReplaceMeWithAnyOfSpec,
    pub description: String,
    pub disputed: bool,
    pub failure_balance_transaction: ReplaceMeWithAnyOfSpec,
    pub failure_code: String,
    pub failure_message: String,
    pub fraud_details: ReplaceMeWithAnyOfSpec,
    pub id: String,
    pub invoice: ReplaceMeWithAnyOfSpec,
    pub level3: Option<Level3>,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub object: String,
    pub on_behalf_of: ReplaceMeWithAnyOfSpec,
    pub outcome: ReplaceMeWithAnyOfSpec,
    pub paid: bool,
    pub payment_intent: ReplaceMeWithAnyOfSpec,
    pub payment_method: String,
    pub payment_method_details: ReplaceMeWithAnyOfSpec,
    pub radar_options: Option<RadarRadarOptions>,
    pub receipt_email: String,
    pub receipt_number: String,
    pub receipt_url: String,
    pub refunded: bool,
    pub refunds: ChargeRefunds,
    pub review: ReplaceMeWithAnyOfSpec,
    pub shipping: ReplaceMeWithAnyOfSpec,
    pub source: ReplaceMeWithAnyOfSpec,
    pub source_transfer: ReplaceMeWithAnyOfSpec,
    pub statement_descriptor: String,
    pub statement_descriptor_suffix: String,
    pub status: ChargeStatus,
    pub transfer: Option<ReplaceMeWithAnyOfSpec>,
    pub transfer_data: ReplaceMeWithAnyOfSpec,
    pub transfer_group: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CustomerAcceptanceType {
    Offline,
    Online,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentLinkCustomerCreation {
    Always,
    IfRequired,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecurringAggregateUsage {
    LastDuringPeriod,
    LastEver,
    Max,
    Sum,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InvoicesResourceLineItemsCreditedItems {
    pub invoice: String,
    pub invoice_line_items: Vec<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CheckoutAuBecsDebitPaymentMethodOptions {
    pub setup_future_usage: Option<
        CheckoutAuBecsDebitPaymentMethodOptionsSetupFutureUsage,
    >,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GelatoVerificationReportOptions {
    pub document: Option<GelatoReportDocumentOptions>,
    pub id_number: Option<GelatoReportIdNumberOptions>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BankConnectionsResourceTransactionRefreshStatus {
    Failed,
    Pending,
    Succeeded,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutAcssDebitPaymentMethodOptionsCurrency {
    Cad,
    Usd,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CreditNoteType {
    PostPayment,
    PrePayment,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AccountBacsDebitPaymentsSettings {
    pub display_name: String,
    pub service_user_number: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinancialConnectionsAccountSubscriptionsItem {
    Transactions,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentityVerificationSessionType {
    Document,
    IdNumber,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FundingInstructionsBankTransferAbaRecord {
    pub account_number: String,
    pub bank_name: String,
    pub routing_number: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentLinksResourceConsentCollectionPromotions {
    Auto,
    None,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutSessionCustomerCreation {
    Always,
    IfRequired,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentLinkPaymentMethodTypesItem {
    Affirm,
    AfterpayClearpay,
    Alipay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Blik,
    Boleto,
    Card,
    Cashapp,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Klarna,
    Konbini,
    Link,
    Oxxo,
    P24,
    Paynow,
    Paypal,
    Pix,
    Promptpay,
    SepaDebit,
    Sofort,
    Swish,
    UsBankAccount,
    WechatPay,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsAfterpayClearpayCaptureMethod {
    Manual,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodOptionsAfterpayClearpay {
    pub capture_method: Option<PaymentMethodOptionsAfterpayClearpayCaptureMethod>,
    pub reference: String,
    pub setup_future_usage: Option<PaymentMethodOptionsAfterpayClearpaySetupFutureUsage>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxProductRegistrationsResourceCountryOptionsEurope {
    pub standard: Option<TaxProductRegistrationsResourceCountryOptionsEuStandard>,
    pub r#type: TaxProductRegistrationsResourceCountryOptionsEuropeType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SubscriptionsResourceBillingCycleAnchorConfig {
    pub day_of_month: i64,
    pub hour: i64,
    pub minute: i64,
    pub month: i64,
    pub second: i64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentPaymentMethodOptionsCardSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MandatePaypal {
    pub billing_agreement_id: String,
    pub payer_id: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LineItem {
    pub amount: i64,
    pub amount_excluding_tax: i64,
    pub currency: String,
    pub description: String,
    pub discount_amounts: Vec<DiscountsResourceDiscountAmount>,
    pub discountable: bool,
    pub discounts: Vec<ReplaceMeWithAnyOfSpec>,
    pub id: String,
    pub invoice_item: Option<ReplaceMeWithAnyOfSpec>,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub object: String,
    pub period: InvoiceLineItemPeriod,
    pub plan: ReplaceMeWithAnyOfSpec,
    pub price: ReplaceMeWithAnyOfSpec,
    pub proration: bool,
    pub proration_details: ReplaceMeWithAnyOfSpec,
    pub quantity: i64,
    pub subscription: ReplaceMeWithAnyOfSpec,
    pub subscription_item: Option<ReplaceMeWithAnyOfSpec>,
    pub tax_amounts: Option<Vec<InvoiceTaxAmount>>,
    pub tax_rates: Option<Vec<TaxRate>>,
    pub r#type: LineItemType,
    pub unit_amount_excluding_tax: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InvoicePaymentMethodOptionsCard {
    pub installments: Option<InvoiceInstallmentsCard>,
    pub request_three_d_secure: InvoicePaymentMethodOptionsCardRequestThreeDSecure,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingTransactionWallet {
    ApplePay,
    GooglePay,
    SamsungPay,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    Checking,
    Savings,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentStatus {
    Canceled,
    Processing,
    RequiresAction,
    RequiresCapture,
    RequiresConfirmation,
    RequiresPaymentMethod,
    Succeeded,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsCardWalletAmexExpressCheckout {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsCustomerBalanceFundingType {
    BankTransfer,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanAggregateUsage {
    LastDuringPeriod,
    LastEver,
    Max,
    Sum,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PortalLoginPage {
    pub enabled: bool,
    pub url: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuoteStatus {
    Accepted,
    Canceled,
    Draft,
    Open,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticapture {
    pub status: PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesFpxPayments {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Balance {
    pub available: Vec<BalanceAmount>,
    pub connect_reserved: Option<Vec<BalanceAmount>>,
    pub instant_available: Option<Vec<BalanceAmountNet>>,
    pub issuing: Option<BalanceDetail>,
    pub livemode: bool,
    pub object: String,
    pub pending: Vec<BalanceAmount>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingAuthorizationPendingRequest {
    pub amount: i64,
    pub amount_details: ReplaceMeWithAnyOfSpec,
    pub currency: String,
    pub is_amount_controllable: bool,
    pub merchant_amount: i64,
    pub merchant_currency: String,
    pub network_risk_score: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingDisputeDuplicateEvidence {
    pub additional_documentation: ReplaceMeWithAnyOfSpec,
    pub card_statement: ReplaceMeWithAnyOfSpec,
    pub cash_receipt: ReplaceMeWithAnyOfSpec,
    pub check_image: ReplaceMeWithAnyOfSpec,
    pub explanation: String,
    pub original_transaction: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LegalEntityCompany {
    pub address: Option<Address>,
    pub address_kana: Option<ReplaceMeWithAnyOfSpec>,
    pub address_kanji: Option<ReplaceMeWithAnyOfSpec>,
    pub directors_provided: Option<bool>,
    pub executives_provided: Option<bool>,
    pub export_license_id: Option<String>,
    pub export_purpose_code: Option<String>,
    pub name: Option<String>,
    pub name_kana: Option<String>,
    pub name_kanji: Option<String>,
    pub owners_provided: Option<bool>,
    pub ownership_declaration: Option<ReplaceMeWithAnyOfSpec>,
    pub phone: Option<String>,
    pub structure: Option<LegalEntityCompanyStructure>,
    pub tax_id_provided: Option<bool>,
    pub tax_id_registrar: Option<String>,
    pub vat_id_provided: Option<bool>,
    pub verification: Option<ReplaceMeWithAnyOfSpec>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentPaymentMethodOptionsLinkCaptureMethod {
    Manual,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsCashapp {
    pub buyer_id: String,
    pub cashtag: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingCardholderCompany {
    pub tax_id_provided: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecurringUsageType {
    Licensed,
    Metered,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
    Challenge,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetupIntentNextAction {
    pub cashapp_handle_redirect_or_display_qr_code: Option<
        PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCode,
    >,
    pub redirect_to_url: Option<SetupIntentNextActionRedirectToUrl>,
    pub r#type: String,
    pub use_stripe_sdk: Option<SetupIntentNextActionUseStripeSdk>,
    pub verify_with_microdeposits: Option<SetupIntentNextActionVerifyWithMicrodeposits>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionSchedulePhaseConfigurationCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionSchedulePhaseConfigurationProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ConnectEmbeddedPayoutsFeatures {
    pub edit_payout_schedule: bool,
    pub instant_payouts: bool,
    pub standard_payouts: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingNetworkTokenWalletProviderSuggestedDecision {
    Approve,
    Decline,
    RequireAuth,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThreeDSecureDetailsChargeResultReason {
    Abandoned,
    Bypassed,
    Canceled,
    CardNotEnrolled,
    NetworkNotSupported,
    ProtocolError,
    Rejected,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetupAttemptPaymentMethodDetailsSepaDebit {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutAcssDebitPaymentMethodOptionsVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxProductResourceCustomerDetails {
    pub address: ReplaceMeWithAnyOfSpec,
    pub address_source: TaxProductResourceCustomerDetailsAddressSource,
    pub ip_address: String,
    pub tax_ids: Vec<TaxProductResourceCustomerDetailsResourceTaxId>,
    pub taxability_override: TaxProductResourceCustomerDetailsTaxabilityOverride,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxRate {
    pub active: bool,
    pub country: String,
    pub created: i64,
    pub description: String,
    pub display_name: String,
    pub effective_percentage: f64,
    pub id: String,
    pub inclusive: bool,
    pub jurisdiction: String,
    pub jurisdiction_level: TaxRateJurisdictionLevel,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub object: String,
    pub percentage: f64,
    pub state: String,
    pub tax_type: TaxRateTaxType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountStatus {
    Closed,
    Open,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryReceivedCreditsResourceStatusTransitions {
    pub posted_at: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodOptionsWechatPay {
    pub app_id: String,
    pub client: PaymentMethodOptionsWechatPayClient,
    pub setup_future_usage: Option<PaymentMethodOptionsWechatPaySetupFutureUsage>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AccountTosAcceptance {
    pub date: Option<i64>,
    pub ip: Option<String>,
    pub service_agreement: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InvoiceItemThresholdReason {
    pub line_item_ids: Vec<String>,
    pub usage_gte: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentPaymentMethodOptions {
    pub acss_debit: Option<PaymentIntentPaymentMethodOptionsAcssDebit>,
    pub affirm: Option<PaymentMethodOptionsAffirm>,
    pub afterpay_clearpay: Option<PaymentMethodOptionsAfterpayClearpay>,
    pub alipay: Option<PaymentMethodOptionsAlipay>,
    pub au_becs_debit: Option<PaymentIntentPaymentMethodOptionsAuBecsDebit>,
    pub bacs_debit: Option<PaymentMethodOptionsBacsDebit>,
    pub bancontact: Option<PaymentMethodOptionsBancontact>,
    pub blik: Option<PaymentIntentPaymentMethodOptionsBlik>,
    pub boleto: Option<PaymentMethodOptionsBoleto>,
    pub card: Option<PaymentIntentPaymentMethodOptionsCard>,
    pub card_present: Option<PaymentMethodOptionsCardPresent>,
    pub cashapp: Option<PaymentMethodOptionsCashapp>,
    pub customer_balance: Option<PaymentMethodOptionsCustomerBalance>,
    pub eps: Option<PaymentIntentPaymentMethodOptionsEps>,
    pub fpx: Option<PaymentMethodOptionsFpx>,
    pub giropay: Option<PaymentMethodOptionsGiropay>,
    pub grabpay: Option<PaymentMethodOptionsGrabpay>,
    pub ideal: Option<PaymentMethodOptionsIdeal>,
    pub interac_present: Option<PaymentMethodOptionsInteracPresent>,
    pub klarna: Option<PaymentMethodOptionsKlarna>,
    pub konbini: Option<PaymentMethodOptionsKonbini>,
    pub link: Option<PaymentIntentPaymentMethodOptionsLink>,
    pub oxxo: Option<PaymentMethodOptionsOxxo>,
    pub p24: Option<PaymentMethodOptionsP24>,
    pub paynow: Option<PaymentMethodOptionsPaynow>,
    pub paypal: Option<PaymentMethodOptionsPaypal>,
    pub pix: Option<PaymentMethodOptionsPix>,
    pub promptpay: Option<PaymentMethodOptionsPromptpay>,
    pub revolut_pay: Option<PaymentMethodOptionsRevolutPay>,
    pub sepa_debit: Option<PaymentIntentPaymentMethodOptionsSepaDebit>,
    pub sofort: Option<PaymentMethodOptionsSofort>,
    pub swish: Option<PaymentIntentPaymentMethodOptionsSwish>,
    pub us_bank_account: Option<PaymentIntentPaymentMethodOptionsUsBankAccount>,
    pub wechat_pay: Option<PaymentMethodOptionsWechatPay>,
    pub zip: Option<PaymentMethodOptionsZip>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionInvoiceCreation {
    pub enabled: bool,
    pub invoice_data: PaymentPagesCheckoutSessionInvoiceSettings,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ShippingRateCurrencyOption {
    pub amount: i64,
    pub tax_behavior: ShippingRateCurrencyOptionTaxBehavior,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryReceivedDebitNetwork {
    Ach,
    Card,
    Stripe,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortalFlowsRetentionType {
    CouponOffer,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PortalCustomerUpdate {
    pub allowed_updates: Vec<PortalCustomerUpdateAllowedUpdatesItem>,
    pub enabled: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SourceTypeSofort {
    pub bank_code: Option<String>,
    pub bank_name: Option<String>,
    pub bic: Option<String>,
    pub country: Option<String>,
    pub iban_last4: Option<String>,
    pub preferred_language: Option<String>,
    pub statement_descriptor: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkedAccountOptionsUsBankAccountPermissionsItem {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FinancialConnectionsAccountOwnership {
    pub created: i64,
    pub id: String,
    pub object: String,
    pub owners: FinancialConnectionsAccountOwnershipOwners,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct QuotesResourceAutomaticTax {
    pub enabled: bool,
    pub liability: ReplaceMeWithAnyOfSpec,
    pub status: QuotesResourceAutomaticTaxStatus,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BalanceAmount {
    pub amount: i64,
    pub currency: String,
    pub source_types: Option<BalanceAmountBySourceType>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EmailSent {
    pub email_sent_at: i64,
    pub email_sent_to: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductRegistrationsResourceCountryOptionsDefaultType {
    Standard,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesAffirmPayments {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ChargeRefunds {
    pub data: Vec<Refund>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutSessionMode {
    Payment,
    Setup,
    Subscription,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CustomerBalanceCustomerBalanceSettings {
    pub reconciliation_mode: CustomerBalanceCustomerBalanceSettingsReconciliationMode,
    pub using_merchant_default: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GelatoDocumentReportError {
    pub code: GelatoDocumentReportErrorCode,
    pub reason: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxProductResourceTaxRateDetails {
    pub country: String,
    pub percentage_decimal: String,
    pub state: String,
    pub tax_type: TaxProductResourceTaxRateDetailsTaxType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalReaderReaderResourceLineItem {
    pub amount: i64,
    pub description: String,
    pub quantity: i64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentPagesCheckoutSessionConsentCollectionPromotions {
    Auto,
    None,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingTransactionType {
    Capture,
    Refund,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentNextActionDisplayBankTransferInstructions {
    pub amount_remaining: i64,
    pub currency: String,
    pub financial_addresses: Option<
        Vec<FundingInstructionsBankTransferFinancialAddress>,
    >,
    pub hosted_instructions_url: String,
    pub reference: String,
    pub r#type: PaymentIntentNextActionDisplayBankTransferInstructionsType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodUsBankAccountBlockedReason {
    BankAccountClosed,
    BankAccountFrozen,
    BankAccountInvalidDetails,
    BankAccountRestricted,
    BankAccountUnusable,
    DebitNotAuthorized,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxRegistration {
    pub active_from: i64,
    pub country: String,
    pub country_options: TaxProductRegistrationsResourceCountryOptions,
    pub created: i64,
    pub expires_at: i64,
    pub id: String,
    pub livemode: bool,
    pub object: String,
    pub status: TaxRegistrationStatus,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetupAttemptPaymentMethodDetailsBoleto {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalConfigurationConfigurationResourceCurrencySpecificConfig {
    pub fixed_amounts: Option<Vec<i64>>,
    pub percentages: Option<Vec<i64>>,
    pub smart_tip_threshold: Option<i64>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TransferData {
    pub amount: Option<i64>,
    pub destination: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThreeDSecureDetailsElectronicCommerceIndicator {
    No_01,
    No_02,
    No_05,
    No_06,
    No_07,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ConnectEmbeddedPaymentsConfig {
    pub enabled: bool,
    pub features: ConnectEmbeddedPaymentsFeatures,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MandateBacsDebit {
    pub network_status: MandateBacsDebitNetworkStatus,
    pub reference: String,
    pub revocation_reason: MandateBacsDebitRevocationReason,
    pub url: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodOptionsKlarna {
    pub capture_method: Option<PaymentMethodOptionsKlarnaCaptureMethod>,
    pub preferred_locale: String,
    pub setup_future_usage: Option<PaymentMethodOptionsKlarnaSetupFutureUsage>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShippingRateCurrencyOptionTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxProductResourceCustomerDetailsResourceTaxId {
    pub r#type: TaxProductResourceCustomerDetailsResourceTaxIdType,
    pub value: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutAuBecsDebitPaymentMethodOptionsSetupFutureUsage {
    None,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentPaymentMethodOptionsSwishSetupFutureUsage {
    None,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NotificationEventRequest {
    pub id: String,
    pub idempotency_key: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingDisputeCanceledEvidenceProductType {
    Merchandise,
    Service,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardShippingStatus {
    Canceled,
    Delivered,
    Failure,
    Pending,
    Returned,
    Shipped,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub amount_subtotal: i64,
    pub amount_total: i64,
    pub application: ReplaceMeWithAnyOfSpec,
    pub application_fee_amount: i64,
    pub application_fee_percent: f64,
    pub automatic_tax: QuotesResourceAutomaticTax,
    pub collection_method: QuoteCollectionMethod,
    pub computed: QuotesResourceComputed,
    pub created: i64,
    pub currency: String,
    pub customer: ReplaceMeWithAnyOfSpec,
    pub default_tax_rates: Option<Vec<ReplaceMeWithAnyOfSpec>>,
    pub description: String,
    pub discounts: Vec<ReplaceMeWithAnyOfSpec>,
    pub expires_at: i64,
    pub footer: String,
    pub from_quote: ReplaceMeWithAnyOfSpec,
    pub header: String,
    pub id: String,
    pub invoice: ReplaceMeWithAnyOfSpec,
    pub invoice_settings: InvoiceSettingQuoteSetting,
    pub line_items: Option<QuoteLineItems>,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub number: String,
    pub object: String,
    pub on_behalf_of: ReplaceMeWithAnyOfSpec,
    pub status: QuoteStatus,
    pub status_transitions: QuotesResourceStatusTransitions,
    pub subscription: ReplaceMeWithAnyOfSpec,
    pub subscription_data: QuotesResourceSubscriptionDataSubscriptionData,
    pub subscription_schedule: ReplaceMeWithAnyOfSpec,
    pub test_clock: ReplaceMeWithAnyOfSpec,
    pub total_details: QuotesResourceTotalDetails,
    pub transfer_data: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesAfterpayClearpayPayments {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingCardApplePay {
    pub eligible: bool,
    pub ineligible_reason: IssuingCardApplePayIneligibleReason,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodOptionsCustomerBalanceBankTransfer {
    pub eu_bank_transfer: Option<PaymentMethodOptionsCustomerBalanceEuBankAccount>,
    pub requested_address_types: Option<
        Vec<PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem>,
    >,
    pub r#type: PaymentMethodOptionsCustomerBalanceBankTransferType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesExtendedAuthorizationExtendedAuthorization {
    pub status: PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesExtendedAuthorizationExtendedAuthorizationStatus,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CardMandatePaymentMethodDetails {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GelatoSessionLastError {
    pub code: GelatoSessionLastErrorCode,
    pub reason: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SourceTypeAchCreditTransfer {
    pub account_number: Option<String>,
    pub bank_name: Option<String>,
    pub fingerprint: Option<String>,
    pub refund_account_holder_name: Option<String>,
    pub refund_account_holder_type: Option<String>,
    pub refund_routing_number: Option<String>,
    pub routing_number: Option<String>,
    pub swift_code: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebit {
    pub custom_mandate_url: Option<String>,
    pub interval_description: String,
    pub payment_schedule: PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule,
    pub transaction_type: PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountType {
    Custom,
    Express,
    Standard,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxProductResourcePostalAddress {
    pub city: String,
    pub country: String,
    pub line1: String,
    pub line2: String,
    pub postal_code: String,
    pub state: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CustomerSubscriptions {
    pub data: Vec<Subscription>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThreeDSecureDetailsChargeElectronicCommerceIndicator {
    No_01,
    No_02,
    No_05,
    No_06,
    No_07,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentTypeSpecificPaymentMethodOptionsClientCaptureMethod {
    Manual,
    ManualPreferred,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CustomerBalanceTransaction {
    pub amount: i64,
    pub created: i64,
    pub credit_note: ReplaceMeWithAnyOfSpec,
    pub currency: String,
    pub customer: ReplaceMeWithAnyOfSpec,
    pub description: String,
    pub ending_balance: i64,
    pub id: String,
    pub invoice: ReplaceMeWithAnyOfSpec,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub object: String,
    pub r#type: CustomerBalanceTransactionType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CheckoutLinkPaymentMethodOptions {
    pub setup_future_usage: Option<CheckoutLinkPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsIdealBic {
    ABNANL2A,
    ASNBNL21,
    BITSNL2A,
    BUNQNL2A,
    FVLBNL22,
    HANDNL2A,
    INGBNL2A,
    KNABNL2H,
    MOYONL21,
    NNBANL2G,
    NTSBDEB1,
    RABONL2U,
    RBRBNL21,
    REVOIE23,
    REVOLT21,
    SNSBNL2A,
    TRIONL2U,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionShippingOption {
    pub shipping_amount: i64,
    pub shipping_rate: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UsageRecord {
    pub id: String,
    pub livemode: bool,
    pub object: String,
    pub quantity: i64,
    pub subscription_item: String,
    pub timestamp: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalReaderReaderResourceReaderAction {
    pub failure_code: String,
    pub failure_message: String,
    pub process_payment_intent: Option<
        TerminalReaderReaderResourceProcessPaymentIntentAction,
    >,
    pub process_setup_intent: Option<
        TerminalReaderReaderResourceProcessSetupIntentAction,
    >,
    pub refund_payment: Option<TerminalReaderReaderResourceRefundPaymentAction>,
    pub set_reader_display: Option<TerminalReaderReaderResourceSetReaderDisplayAction>,
    pub status: TerminalReaderReaderResourceReaderActionStatus,
    pub r#type: TerminalReaderReaderResourceReaderActionType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewOpenedReason {
    Manual,
    Rule,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodIdealBank {
    AbnAmro,
    AsnBank,
    Bunq,
    Handelsbanken,
    Ing,
    Knab,
    Moneyou,
    N26,
    Nn,
    Rabobank,
    Regiobank,
    Revolut,
    SnsBank,
    TriodosBank,
    VanLanschot,
    Yoursafe,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LegalEntityDob {
    pub day: i64,
    pub month: i64,
    pub year: i64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefundReason {
    Duplicate,
    ExpiredUncapturedCharge,
    Fraudulent,
    RequestedByCustomer,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DisputePaymentMethodDetails {
    pub card: ReplaceMeWithAnyOfSpec,
    pub r#type: DisputePaymentMethodDetailsType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentLinksResourceInvoiceCreation {
    pub enabled: bool,
    pub invoice_data: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetupIntent {
    pub application: ReplaceMeWithAnyOfSpec,
    pub attach_to_self: Option<bool>,
    pub automatic_payment_methods: ReplaceMeWithAnyOfSpec,
    pub cancellation_reason: SetupIntentCancellationReason,
    pub client_secret: String,
    pub created: i64,
    pub customer: ReplaceMeWithAnyOfSpec,
    pub description: String,
    pub flow_directions: Vec<SetupIntentFlowDirectionsItem>,
    pub id: String,
    pub last_setup_error: ReplaceMeWithAnyOfSpec,
    pub latest_attempt: ReplaceMeWithAnyOfSpec,
    pub livemode: bool,
    pub mandate: ReplaceMeWithAnyOfSpec,
    pub metadata: HashMap<String, String>,
    pub next_action: ReplaceMeWithAnyOfSpec,
    pub object: String,
    pub on_behalf_of: ReplaceMeWithAnyOfSpec,
    pub payment_method: ReplaceMeWithAnyOfSpec,
    pub payment_method_configuration_details: ReplaceMeWithAnyOfSpec,
    pub payment_method_options: ReplaceMeWithAnyOfSpec,
    pub payment_method_types: Vec<String>,
    pub single_use_mandate: ReplaceMeWithAnyOfSpec,
    pub status: SetupIntentStatus,
    pub usage: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CheckoutP24PaymentMethodOptions {
    pub setup_future_usage: Option<CheckoutP24PaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryFinancialAccountsResourceBalance {
    pub cash: TreasuryFinancialAccountsResourceBalanceCash,
    pub inbound_pending: TreasuryFinancialAccountsResourceBalanceInboundPending,
    pub outbound_pending: TreasuryFinancialAccountsResourceBalanceOutboundPending,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupAttemptPaymentMethodDetailsCardWalletType {
    ApplePay,
    GooglePay,
    Link,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Level3LineItems {
    pub discount_amount: i64,
    pub product_code: String,
    pub product_description: String,
    pub quantity: i64,
    pub tax_amount: i64,
    pub unit_cost: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetupAttemptPaymentMethodDetailsCardPresent {
    pub generated_card: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod {
    Cancel,
    CreateInvoice,
    Pause,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FundingInstructionsBankTransferFinancialAddress {
    pub aba: Option<FundingInstructionsBankTransferAbaRecord>,
    pub iban: Option<FundingInstructionsBankTransferIbanRecord>,
    pub sort_code: Option<FundingInstructionsBankTransferSortCodeRecord>,
    pub spei: Option<FundingInstructionsBankTransferSpeiRecord>,
    pub supported_networks: Option<
        Vec<FundingInstructionsBankTransferFinancialAddressSupportedNetworksItem>,
    >,
    pub swift: Option<FundingInstructionsBankTransferSwiftRecord>,
    pub r#type: FundingInstructionsBankTransferFinancialAddressType,
    pub zengin: Option<FundingInstructionsBankTransferZenginRecord>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxProductResourceTaxCalculationShippingCost {
    pub amount: i64,
    pub amount_tax: i64,
    pub shipping_rate: Option<String>,
    pub tax_behavior: TaxProductResourceTaxCalculationShippingCostTaxBehavior,
    pub tax_breakdown: Option<Vec<TaxProductResourceLineItemTaxBreakdown>>,
    pub tax_code: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodCardWalletGooglePay {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeletedTerminalLocation {
    pub deleted: bool,
    pub id: String,
    pub object: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanUsageType {
    Licensed,
    Metered,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingCardholder {
    pub billing: IssuingCardholderAddress,
    pub company: ReplaceMeWithAnyOfSpec,
    pub created: i64,
    pub email: String,
    pub id: String,
    pub individual: ReplaceMeWithAnyOfSpec,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub name: String,
    pub object: String,
    pub phone_number: String,
    pub preferred_locales: Vec<IssuingCardholderPreferredLocalesItem>,
    pub requirements: IssuingCardholderRequirements,
    pub spending_controls: ReplaceMeWithAnyOfSpec,
    pub status: IssuingCardholderStatus,
    pub r#type: IssuingCardholderType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewClosedReason {
    Approved,
    Disputed,
    Redacted,
    Refunded,
    RefundedAsFraud,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CardIssuingAccountTermsOfService {
    pub date: i64,
    pub ip: String,
    pub user_agent: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountsResourceAbaToggleSettingsStatus {
    Active,
    Pending,
    Restricted,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentNextActionKonbiniLawson {
    pub confirmation_number: Option<String>,
    pub payment_code: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodUsBankAccountStatusDetails {
    pub blocked: Option<PaymentMethodUsBankAccountBlocked>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxDeductedAtSource {
    pub id: String,
    pub object: String,
    pub period_end: i64,
    pub period_start: i64,
    pub tax_deduction_account_number: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingAuthorizationAuthenticationExemptionType {
    LowValueTransaction,
    TransactionRiskAnalysis,
    Unknown,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryFinancialAccountFeatures {
    pub card_issuing: Option<TreasuryFinancialAccountsResourceToggleSettings>,
    pub deposit_insurance: Option<TreasuryFinancialAccountsResourceToggleSettings>,
    pub financial_addresses: Option<
        TreasuryFinancialAccountsResourceFinancialAddressesFeatures,
    >,
    pub inbound_transfers: Option<TreasuryFinancialAccountsResourceInboundTransfers>,
    pub intra_stripe_flows: Option<TreasuryFinancialAccountsResourceToggleSettings>,
    pub object: String,
    pub outbound_payments: Option<TreasuryFinancialAccountsResourceOutboundPayments>,
    pub outbound_transfers: Option<TreasuryFinancialAccountsResourceOutboundTransfers>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryFinancialAccountsResourceAbaToggleSettings {
    pub requested: bool,
    pub status: TreasuryFinancialAccountsResourceAbaToggleSettingsStatus,
    pub status_details: Vec<
        TreasuryFinancialAccountsResourceTogglesSettingStatusDetails,
    >,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodBancontact {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CountrySpec {
    pub default_currency: String,
    pub id: String,
    pub object: String,
    pub supported_bank_account_currencies: CountrySpecSupportedBankAccountCurrencies,
    pub supported_payment_currencies: Vec<String>,
    pub supported_payment_methods: Vec<String>,
    pub supported_transfer_countries: Vec<String>,
    pub verification_fields: CountrySpecVerificationFields,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntent {
    pub amount: i64,
    pub amount_capturable: i64,
    pub amount_details: Option<PaymentFlowsAmountDetails>,
    pub amount_received: i64,
    pub application: ReplaceMeWithAnyOfSpec,
    pub application_fee_amount: i64,
    pub automatic_payment_methods: ReplaceMeWithAnyOfSpec,
    pub canceled_at: i64,
    pub cancellation_reason: PaymentIntentCancellationReason,
    pub capture_method: PaymentIntentCaptureMethod,
    pub client_secret: String,
    pub confirmation_method: PaymentIntentConfirmationMethod,
    pub created: i64,
    pub currency: String,
    pub customer: ReplaceMeWithAnyOfSpec,
    pub description: String,
    pub id: String,
    pub invoice: ReplaceMeWithAnyOfSpec,
    pub last_payment_error: ReplaceMeWithAnyOfSpec,
    pub latest_charge: ReplaceMeWithAnyOfSpec,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub next_action: ReplaceMeWithAnyOfSpec,
    pub object: String,
    pub on_behalf_of: ReplaceMeWithAnyOfSpec,
    pub payment_method: ReplaceMeWithAnyOfSpec,
    pub payment_method_configuration_details: ReplaceMeWithAnyOfSpec,
    pub payment_method_options: ReplaceMeWithAnyOfSpec,
    pub payment_method_types: Vec<String>,
    pub processing: ReplaceMeWithAnyOfSpec,
    pub receipt_email: String,
    pub review: ReplaceMeWithAnyOfSpec,
    pub setup_future_usage: PaymentIntentSetupFutureUsage,
    pub shipping: ReplaceMeWithAnyOfSpec,
    pub source: ReplaceMeWithAnyOfSpec,
    pub statement_descriptor: String,
    pub statement_descriptor_suffix: String,
    pub status: PaymentIntentStatus,
    pub transfer_data: ReplaceMeWithAnyOfSpec,
    pub transfer_group: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutSofortPaymentMethodOptionsSetupFutureUsage {
    None,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InboundTransfersPaymentMethodDetailsUsBankAccount {
    pub account_holder_type: InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType,
    pub account_type: InboundTransfersPaymentMethodDetailsUsBankAccountAccountType,
    pub bank_name: String,
    pub fingerprint: String,
    pub last4: String,
    pub network: InboundTransfersPaymentMethodDetailsUsBankAccountNetwork,
    pub routing_number: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardholderRequirementsPastDueItem {
    CompanyTaxId,
    IndividualCardIssuingUserTermsAcceptanceDate,
    IndividualCardIssuingUserTermsAcceptanceIp,
    IndividualDobDay,
    IndividualDobMonth,
    IndividualDobYear,
    IndividualFirstName,
    IndividualLastName,
    IndividualVerificationDocument,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsAchDebitAccountHolderType {
    Company,
    Individual,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutGiropayPaymentMethodOptionsSetupFutureUsage {
    None,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreditNoteTaxAmount {
    pub amount: i64,
    pub inclusive: bool,
    pub tax_rate: ReplaceMeWithAnyOfSpec,
    pub taxability_reason: CreditNoteTaxAmountTaxabilityReason,
    pub taxable_amount: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FundingInstructionsBankTransferSwiftRecord {
    pub account_number: String,
    pub bank_name: String,
    pub swift_code: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SigmaScheduledQueryRunError {
    pub message: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PlanTier {
    pub flat_amount: i64,
    pub flat_amount_decimal: String,
    pub unit_amount: i64,
    pub unit_amount_decimal: String,
    pub up_to: i64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrencyOptionTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaypalSellerProtectionDisputeCategoriesItem {
    Fraudulent,
    ProductNotReceived,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ClimateRemovalsBeneficiary {
    pub public_name: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeletedApplication {
    pub deleted: bool,
    pub id: String,
    pub name: String,
    pub object: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalReaderReaderResourceSetReaderDisplayActionType {
    Cart,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LegalEntityCompanyStructure {
    FreeZoneEstablishment,
    FreeZoneLlc,
    GovernmentInstrumentality,
    GovernmentalUnit,
    IncorporatedNonProfit,
    IncorporatedPartnership,
    LimitedLiabilityPartnership,
    Llc,
    MultiMemberLlc,
    PrivateCompany,
    PrivateCorporation,
    PrivatePartnership,
    PublicCompany,
    PublicCorporation,
    PublicPartnership,
    RegisteredCharity,
    SingleMemberLlc,
    SoleEstablishment,
    SoleProprietorship,
    TaxExemptGovernmentInstrumentality,
    UnincorporatedAssociation,
    UnincorporatedNonProfit,
    UnincorporatedPartnership,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentLinksResourceCustomText {
    pub after_submit: ReplaceMeWithAnyOfSpec,
    pub shipping_address: ReplaceMeWithAnyOfSpec,
    pub submit: ReplaceMeWithAnyOfSpec,
    pub terms_of_service_acceptance: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentPaymentMethodOptionsUsBankAccount {
    pub financial_connections: Option<LinkedAccountOptionsUsBankAccount>,
    pub mandate_options: Option<PaymentMethodOptionsUsBankAccountMandateOptions>,
    pub preferred_settlement_speed: Option<
        PaymentIntentPaymentMethodOptionsUsBankAccountPreferredSettlementSpeed,
    >,
    pub setup_future_usage: Option<
        PaymentIntentPaymentMethodOptionsUsBankAccountSetupFutureUsage,
    >,
    pub verification_method: Option<
        PaymentIntentPaymentMethodOptionsUsBankAccountVerificationMethod,
    >,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsPromptpaySetupFutureUsage {
    None,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LegalEntityCompanyVerification {
    pub document: LegalEntityCompanyVerificationDocument,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingNetworkTokenDeviceType {
    Other,
    Phone,
    Watch,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ClimateProduct {
    pub created: i64,
    pub current_prices_per_metric_ton: ClimateProductCurrentPricesPerMetricTon,
    pub delivery_year: i64,
    pub id: String,
    pub livemode: bool,
    pub metric_tons_available: String,
    pub name: String,
    pub object: String,
    pub suppliers: Vec<ClimateSupplier>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GelatoDataVerifiedOutputsDate {
    pub day: i64,
    pub month: i64,
    pub year: i64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BankConnectionsResourceAccountholderType {
    Account,
    Customer,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductRegistrationsResourceCountryOptionsEuropeType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryReceivedDebitsResourceDebitReversalLinkedFlows {
    pub issuing_dispute: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TransformQuantity {
    pub divide_by: i64,
    pub round: TransformQuantityRound,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingTransactionReceiptData {
    pub description: String,
    pub quantity: f64,
    pub total: i64,
    pub unit_cost: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CustomerSessionResourceComponents {
    pub buy_button: CustomerSessionResourceComponentsResourceBuyButton,
    pub pricing_table: CustomerSessionResourceComponentsResourcePricingTable,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentCardProcessing {
    pub customer_notification: Option<PaymentIntentProcessingCustomerNotification>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsOxxoSetupFutureUsage {
    None,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AccountLink {
    pub created: i64,
    pub expires_at: i64,
    pub object: String,
    pub url: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InvoicePaymentMethodOptionsAcssDebitMandateOptions {
    pub transaction_type: InvoicePaymentMethodOptionsAcssDebitMandateOptionsTransactionType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PortalFlowsFlow {
    pub after_completion: PortalFlowsFlowAfterCompletion,
    pub subscription_cancel: ReplaceMeWithAnyOfSpec,
    pub subscription_update: ReplaceMeWithAnyOfSpec,
    pub subscription_update_confirm: ReplaceMeWithAnyOfSpec,
    pub r#type: PortalFlowsFlowType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions {
    pub permissions: Option<
        Vec<InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissionsItem>,
    >,
    pub prefetch: Vec<
        InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPrefetchItem,
    >,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardShippingCarrier {
    Dhl,
    Fedex,
    RoyalMail,
    Usps,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GelatoIdNumberReportStatus {
    Unverified,
    Verified,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork {
    Ach,
    UsDomesticWire,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CashBalance {
    pub available: CashBalanceAvailable,
    pub customer: String,
    pub livemode: bool,
    pub object: String,
    pub settings: CustomerBalanceCustomerBalanceSettings,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsFpxSetupFutureUsage {
    None,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesBlikPayments {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OutboundTransfersPaymentMethodDetails {
    pub billing_details: TreasurySharedResourceBillingDetails,
    pub r#type: OutboundTransfersPaymentMethodDetailsType,
    pub us_bank_account: Option<OutboundTransfersPaymentMethodDetailsUsBankAccount>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentFlowsAutomaticPaymentMethodsSetupIntent {
    pub allow_redirects: Option<
        PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects,
    >,
    pub enabled: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsSwish {
    pub fingerprint: String,
    pub payment_reference: String,
    pub verified_phone_last4: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsMultibanco {
    pub entity: String,
    pub reference: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortalSubscriptionCancelProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CheckoutOxxoPaymentMethodOptions {
    pub expires_after_days: i64,
    pub setup_future_usage: Option<CheckoutOxxoPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesUsBankAccountAchPayments {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ThreeDSecureDetails {
    pub authentication_flow: ThreeDSecureDetailsAuthenticationFlow,
    pub electronic_commerce_indicator: ThreeDSecureDetailsElectronicCommerceIndicator,
    pub result: ThreeDSecureDetailsResult,
    pub result_reason: ThreeDSecureDetailsResultReason,
    pub transaction_id: String,
    pub version: ThreeDSecureDetailsVersion,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryDebitReversal {
    pub amount: i64,
    pub created: i64,
    pub currency: String,
    pub financial_account: String,
    pub hosted_regulatory_receipt_url: String,
    pub id: String,
    pub linked_flows: ReplaceMeWithAnyOfSpec,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub network: TreasuryDebitReversalNetwork,
    pub object: String,
    pub received_debit: String,
    pub status: TreasuryDebitReversalStatus,
    pub status_transitions: TreasuryReceivedDebitsResourceStatusTransitions,
    pub transaction: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryFinancialAccountsResourceAchToggleSettings {
    pub requested: bool,
    pub status: TreasuryFinancialAccountsResourceAchToggleSettingsStatus,
    pub status_details: Vec<
        TreasuryFinancialAccountsResourceTogglesSettingStatusDetails,
    >,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsCard {
    pub amount_authorized: i64,
    pub brand: String,
    pub capture_before: Option<i64>,
    pub checks: ReplaceMeWithAnyOfSpec,
    pub country: String,
    pub description: Option<String>,
    pub exp_month: i64,
    pub exp_year: i64,
    pub extended_authorization: Option<
        PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesExtendedAuthorizationExtendedAuthorization,
    >,
    pub fingerprint: Option<String>,
    pub funding: String,
    pub iin: Option<String>,
    pub incremental_authorization: Option<
        PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesIncrementalAuthorizationIncrementalAuthorization,
    >,
    pub installments: ReplaceMeWithAnyOfSpec,
    pub issuer: Option<String>,
    pub last4: String,
    pub mandate: String,
    pub moto: Option<bool>,
    pub multicapture: Option<
        PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticapture,
    >,
    pub network: String,
    pub network_token: Option<ReplaceMeWithAnyOfSpec>,
    pub overcapture: Option<
        PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercapture,
    >,
    pub three_d_secure: ReplaceMeWithAnyOfSpec,
    pub wallet: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ReserveTransaction {
    pub amount: i64,
    pub currency: String,
    pub description: String,
    pub id: String,
    pub object: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesSepaDebitPayments {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FinancialConnectionsTransaction {
    pub account: String,
    pub amount: i64,
    pub currency: String,
    pub description: String,
    pub id: String,
    pub livemode: bool,
    pub object: String,
    pub status: FinancialConnectionsTransactionStatus,
    pub status_transitions: BankConnectionsResourceTransactionResourceStatusTransitions,
    pub transacted_at: i64,
    pub transaction_refresh: String,
    pub updated: i64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodFpxBank {
    AffinBank,
    Agrobank,
    AllianceBank,
    Ambank,
    BankIslam,
    BankMuamalat,
    BankOfChina,
    BankRakyat,
    Bsn,
    Cimb,
    DeutscheBank,
    HongLeongBank,
    Hsbc,
    Kfh,
    Maybank2e,
    Maybank2u,
    Ocbc,
    PbEnterprise,
    PublicBank,
    Rhb,
    StandardChartered,
    Uob,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer {
    pub eu_bank_transfer: Option<
        CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceEuBankTransfer,
    >,
    pub gb_bank_transfer: Option<
        CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransfer,
    >,
    pub jp_bank_transfer: Option<
        CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceJpBankTransfer,
    >,
    pub reference: String,
    pub r#type: CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType,
    pub us_bank_transfer: Option<
        CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceUsBankTransfer,
    >,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodCardChecks {
    pub address_line1_check: String,
    pub address_postal_code_check: String,
    pub cvc_check: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardSpendingLimitInterval {
    AllTime,
    Daily,
    Monthly,
    PerAuthorization,
    Weekly,
    Yearly,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodConfiguration {
    pub acss_debit: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub active: bool,
    pub affirm: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub afterpay_clearpay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub alipay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub apple_pay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub application: String,
    pub au_becs_debit: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub bacs_debit: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub bancontact: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub blik: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub boleto: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub card: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub cartes_bancaires: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub cashapp: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub customer_balance: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub eps: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub fpx: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub giropay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub google_pay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub grabpay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub id: String,
    pub ideal: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub is_default: bool,
    pub jcb: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub klarna: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub konbini: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub link: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub livemode: bool,
    pub name: String,
    pub object: String,
    pub oxxo: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub p24: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub parent: String,
    pub paynow: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub paypal: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub promptpay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub revolut_pay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub sepa_debit: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub sofort: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub us_bank_account: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub wechat_pay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BillingPortalSession {
    pub configuration: ReplaceMeWithAnyOfSpec,
    pub created: i64,
    pub customer: String,
    pub flow: ReplaceMeWithAnyOfSpec,
    pub id: String,
    pub livemode: bool,
    pub locale: BillingPortalSessionLocale,
    pub object: String,
    pub on_behalf_of: String,
    pub return_url: String,
    pub url: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentFlowsPrivatePaymentMethodsKlarnaDob {
    pub day: i64,
    pub month: i64,
    pub year: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodOptionsBoleto {
    pub expires_after_days: i64,
    pub setup_future_usage: Option<PaymentMethodOptionsBoletoSetupFutureUsage>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionPaymentMethodReuseAgreement {
    pub position: PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BankConnectionsResourceBalanceApiResourceCashBalance {
    pub available: BankConnectionsResourceBalanceApiResourceCashBalanceAvailable,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDomain {
    pub apple_pay: PaymentMethodDomainResourcePaymentMethodStatus,
    pub created: i64,
    pub domain_name: String,
    pub enabled: bool,
    pub google_pay: PaymentMethodDomainResourcePaymentMethodStatus,
    pub id: String,
    pub link: PaymentMethodDomainResourcePaymentMethodStatus,
    pub livemode: bool,
    pub object: String,
    pub paypal: PaymentMethodDomainResourcePaymentMethodStatus,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesBancontactPayments {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ReportingReportRun {
    pub created: i64,
    pub error: String,
    pub id: String,
    pub livemode: bool,
    pub object: String,
    pub parameters: FinancialReportingFinanceReportRunRunParameters,
    pub report_type: String,
    pub result: ReplaceMeWithAnyOfSpec,
    pub status: String,
    pub succeeded_at: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetupAttemptPaymentMethodDetailsKlarna {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SourceTypeMultibanco {
    pub entity: Option<String>,
    pub reference: Option<String>,
    pub refund_account_holder_address_city: Option<String>,
    pub refund_account_holder_address_country: Option<String>,
    pub refund_account_holder_address_line1: Option<String>,
    pub refund_account_holder_address_line2: Option<String>,
    pub refund_account_holder_address_postal_code: Option<String>,
    pub refund_account_holder_address_state: Option<String>,
    pub refund_account_holder_name: Option<String>,
    pub refund_iban: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsKlarna {
    pub payment_method_category: String,
    pub preferred_locale: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PortalFlowsFlowSubscriptionUpdate {
    pub subscription: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalReaderReaderResourceProcessSetupConfig {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingTokenStatus {
    Active,
    Deleted,
    Requested,
    Suspended,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThreeDSecureDetailsChargeVersion {
    No_102,
    No_210,
    No_220,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilePurpose {
    AccountRequirement,
    AdditionalVerification,
    BusinessIcon,
    BusinessLogo,
    CustomerSignature,
    DisputeEvidence,
    DocumentProviderIdentityDocument,
    FinanceReportRun,
    IdentityDocument,
    IdentityDocumentDownloadable,
    PciDocument,
    Selfie,
    SigmaScheduledQuery,
    TaxDocumentUserUpload,
    TerminalReaderSplashscreen,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentNextActionSwishHandleRedirectOrDisplayQrCode {
    pub hosted_instructions_url: Option<String>,
    pub mobile_auth_url: Option<String>,
    pub qr_code: Option<PaymentIntentNextActionSwishQrCode>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsCardInstallmentsPlanInterval {
    Month,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsCardPresent {
    pub amount_authorized: i64,
    pub brand: String,
    pub capture_before: Option<i64>,
    pub cardholder_name: String,
    pub country: String,
    pub description: Option<String>,
    pub emv_auth_data: String,
    pub exp_month: i64,
    pub exp_year: i64,
    pub fingerprint: String,
    pub funding: String,
    pub generated_card: String,
    pub iin: Option<String>,
    pub incremental_authorization_supported: bool,
    pub issuer: Option<String>,
    pub last4: String,
    pub network: String,
    pub offline: ReplaceMeWithAnyOfSpec,
    pub overcapture_supported: bool,
    pub read_method: PaymentMethodDetailsCardPresentReadMethod,
    pub receipt: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodType {
    AcssDebit,
    Affirm,
    AfterpayClearpay,
    Alipay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Blik,
    Boleto,
    Card,
    CardPresent,
    Cashapp,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    InteracPresent,
    Klarna,
    Konbini,
    Link,
    Oxxo,
    P24,
    Paynow,
    Paypal,
    Pix,
    Promptpay,
    RevolutPay,
    SepaDebit,
    Sofort,
    Swish,
    UsBankAccount,
    WechatPay,
    Zip,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AccountRequirementsAlternative {
    pub alternative_fields_due: Vec<String>,
    pub original_fields_due: Vec<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionInvoiceSettings {
    pub account_tax_ids: Vec<ReplaceMeWithAnyOfSpec>,
    pub custom_fields: Vec<InvoiceSettingCustomField>,
    pub description: String,
    pub footer: String,
    pub issuer: ReplaceMeWithAnyOfSpec,
    pub metadata: HashMap<String, String>,
    pub rendering_options: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesBoletoPayments {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionAutomaticTax {
    pub enabled: bool,
    pub liability: ReplaceMeWithAnyOfSpec,
    pub status: PaymentPagesCheckoutSessionAutomaticTaxStatus,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingAuthorizationAuthenticationExemption {
    pub claimed_by: IssuingAuthorizationAuthenticationExemptionClaimedBy,
    pub r#type: IssuingAuthorizationAuthenticationExemptionType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingCardholderSpendingLimit {
    pub amount: i64,
    pub categories: Vec<IssuingCardholderSpendingLimitCategoriesItem>,
    pub interval: IssuingCardholderSpendingLimitInterval,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsWechatPaySetupFutureUsage {
    None,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeletedTestHelpersTestClock {
    pub deleted: bool,
    pub id: String,
    pub object: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingCardholderCardIssuing {
    pub user_terms_acceptance: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountsResourceToggleSettingsStatus {
    Active,
    Pending,
    Restricted,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FundingInstructionsBankTransferIbanRecord {
    pub account_holder_name: String,
    pub bic: String,
    pub country: String,
    pub iban: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodBlik {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SubscriptionsResourcePauseCollection {
    pub behavior: SubscriptionsResourcePauseCollectionBehavior,
    pub resumes_at: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SourceTypeIdeal {
    pub bank: Option<String>,
    pub bic: Option<String>,
    pub iban_last4: Option<String>,
    pub statement_descriptor: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BalanceAmountNet {
    pub amount: i64,
    pub currency: String,
    pub source_types: Option<BalanceAmountBySourceType>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UsageRecordSummary {
    pub id: String,
    pub invoice: String,
    pub livemode: bool,
    pub object: String,
    pub period: Period,
    pub subscription_item: String,
    pub total_usage: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SourceTransactionChfCreditTransferData {
    pub reference: Option<String>,
    pub sender_address_country: Option<String>,
    pub sender_address_line1: Option<String>,
    pub sender_iban: Option<String>,
    pub sender_name: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    None,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct QuotesResourceComputed {
    pub recurring: ReplaceMeWithAnyOfSpec,
    pub upfront: QuotesResourceUpfront,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CheckoutFpxPaymentMethodOptions {
    pub setup_future_usage: Option<CheckoutFpxPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinancialConnectionsAccountSubcategory {
    Checking,
    CreditCard,
    LineOfCredit,
    Mortgage,
    Other,
    Savings,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentNextActionDisplayOxxoDetails {
    pub expires_after: i64,
    pub hosted_voucher_url: String,
    pub number: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetupAttemptPaymentMethodDetails {
    pub acss_debit: Option<SetupAttemptPaymentMethodDetailsAcssDebit>,
    pub au_becs_debit: Option<SetupAttemptPaymentMethodDetailsAuBecsDebit>,
    pub bacs_debit: Option<SetupAttemptPaymentMethodDetailsBacsDebit>,
    pub bancontact: Option<SetupAttemptPaymentMethodDetailsBancontact>,
    pub boleto: Option<SetupAttemptPaymentMethodDetailsBoleto>,
    pub card: Option<SetupAttemptPaymentMethodDetailsCard>,
    pub card_present: Option<SetupAttemptPaymentMethodDetailsCardPresent>,
    pub cashapp: Option<SetupAttemptPaymentMethodDetailsCashapp>,
    pub ideal: Option<SetupAttemptPaymentMethodDetailsIdeal>,
    pub klarna: Option<SetupAttemptPaymentMethodDetailsKlarna>,
    pub link: Option<SetupAttemptPaymentMethodDetailsLink>,
    pub paypal: Option<SetupAttemptPaymentMethodDetailsPaypal>,
    pub sepa_debit: Option<SetupAttemptPaymentMethodDetailsSepaDebit>,
    pub sofort: Option<SetupAttemptPaymentMethodDetailsSofort>,
    pub r#type: String,
    pub us_bank_account: Option<SetupAttemptPaymentMethodDetailsUsBankAccount>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClimateOrderStatus {
    AwaitingFunds,
    Canceled,
    Confirmed,
    Delivered,
    Open,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercapture {
    pub maximum_amount_capturable: i64,
    pub status: PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercaptureStatus,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ClimateOrder {
    pub amount_fees: i64,
    pub amount_subtotal: i64,
    pub amount_total: i64,
    pub beneficiary: Option<ClimateRemovalsBeneficiary>,
    pub canceled_at: i64,
    pub cancellation_reason: ClimateOrderCancellationReason,
    pub certificate: String,
    pub confirmed_at: i64,
    pub created: i64,
    pub currency: String,
    pub delayed_at: i64,
    pub delivered_at: i64,
    pub delivery_details: Vec<ClimateRemovalsOrderDeliveries>,
    pub expected_delivery_year: i64,
    pub id: String,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub metric_tons: String,
    pub object: String,
    pub product: ReplaceMeWithAnyOfSpec,
    pub product_substituted_at: i64,
    pub status: ClimateOrderStatus,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetails {
    pub ach_credit_transfer: Option<PaymentMethodDetailsAchCreditTransfer>,
    pub ach_debit: Option<PaymentMethodDetailsAchDebit>,
    pub acss_debit: Option<PaymentMethodDetailsAcssDebit>,
    pub affirm: Option<PaymentMethodDetailsAffirm>,
    pub afterpay_clearpay: Option<PaymentMethodDetailsAfterpayClearpay>,
    pub alipay: Option<PaymentFlowsPrivatePaymentMethodsAlipayDetails>,
    pub au_becs_debit: Option<PaymentMethodDetailsAuBecsDebit>,
    pub bacs_debit: Option<PaymentMethodDetailsBacsDebit>,
    pub bancontact: Option<PaymentMethodDetailsBancontact>,
    pub blik: Option<PaymentMethodDetailsBlik>,
    pub boleto: Option<PaymentMethodDetailsBoleto>,
    pub card: Option<PaymentMethodDetailsCard>,
    pub card_present: Option<PaymentMethodDetailsCardPresent>,
    pub cashapp: Option<PaymentMethodDetailsCashapp>,
    pub customer_balance: Option<PaymentMethodDetailsCustomerBalance>,
    pub eps: Option<PaymentMethodDetailsEps>,
    pub fpx: Option<PaymentMethodDetailsFpx>,
    pub giropay: Option<PaymentMethodDetailsGiropay>,
    pub grabpay: Option<PaymentMethodDetailsGrabpay>,
    pub ideal: Option<PaymentMethodDetailsIdeal>,
    pub interac_present: Option<PaymentMethodDetailsInteracPresent>,
    pub klarna: Option<PaymentMethodDetailsKlarna>,
    pub konbini: Option<PaymentMethodDetailsKonbini>,
    pub link: Option<PaymentMethodDetailsLink>,
    pub multibanco: Option<PaymentMethodDetailsMultibanco>,
    pub oxxo: Option<PaymentMethodDetailsOxxo>,
    pub p24: Option<PaymentMethodDetailsP24>,
    pub paynow: Option<PaymentMethodDetailsPaynow>,
    pub paypal: Option<PaymentMethodDetailsPaypal>,
    pub pix: Option<PaymentMethodDetailsPix>,
    pub promptpay: Option<PaymentMethodDetailsPromptpay>,
    pub revolut_pay: Option<PaymentMethodDetailsRevolutPay>,
    pub sepa_credit_transfer: Option<PaymentMethodDetailsSepaCreditTransfer>,
    pub sepa_debit: Option<PaymentMethodDetailsSepaDebit>,
    pub sofort: Option<PaymentMethodDetailsSofort>,
    pub stripe_account: Option<PaymentMethodDetailsStripeAccount>,
    pub swish: Option<PaymentMethodDetailsSwish>,
    pub r#type: String,
    pub us_bank_account: Option<PaymentMethodDetailsUsBankAccount>,
    pub wechat: Option<PaymentMethodDetailsWechat>,
    pub wechat_pay: Option<PaymentMethodDetailsWechatPay>,
    pub zip: Option<PaymentMethodDetailsZip>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductRegistrationsResourceCountryOptionsCanadaType {
    ProvinceStandard,
    Simplified,
    Standard,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryOutboundTransfersResourceReturnedDetailsCode {
    AccountClosed,
    AccountFrozen,
    BankAccountRestricted,
    BankOwnershipChanged,
    Declined,
    IncorrectAccountHolderName,
    InvalidAccountNumber,
    InvalidCurrency,
    NoAccount,
    Other,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingDisputeNotReceivedEvidenceProductType {
    Merchandise,
    Service,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsBoleto {
    pub tax_id: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutSessionSubmitType {
    Auto,
    Book,
    Donate,
    Pay,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsKonbiniStore {
    pub chain: PaymentMethodDetailsKonbiniStoreChain,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryFinancialAccountsResourceToggleSettings {
    pub requested: bool,
    pub status: TreasuryFinancialAccountsResourceToggleSettingsStatus,
    pub status_details: Vec<
        TreasuryFinancialAccountsResourceTogglesSettingStatusDetails,
    >,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InvoiceMandateOptionsCard {
    pub amount: i64,
    pub amount_type: InvoiceMandateOptionsCardAmountType,
    pub description: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesPromptpayPayments {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ClimateRemovalsLocation {
    pub city: String,
    pub country: String,
    pub latitude: f64,
    pub longitude: f64,
    pub region: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductResourceTaxTransactionShippingCostTaxBehavior {
    Exclusive,
    Inclusive,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinancialConnectionsAccountCategory {
    Cash,
    Credit,
    Investment,
    Other,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryReceivedDebitStatus {
    Failed,
    Succeeded,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupAttemptPaymentMethodDetailsIdealBic {
    ABNANL2A,
    ASNBNL21,
    BITSNL2A,
    BUNQNL2A,
    FVLBNL22,
    HANDNL2A,
    INGBNL2A,
    KNABNL2H,
    MOYONL21,
    NNBANL2G,
    NTSBDEB1,
    RABONL2U,
    RBRBNL21,
    REVOIE23,
    REVOLT21,
    SNSBNL2A,
    TRIONL2U,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxProductRegistrationsResourceCountryOptionsCaProvinceStandard {
    pub province: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryInboundTransfersResourceFailureDetailsCode {
    AccountClosed,
    AccountFrozen,
    BankAccountRestricted,
    BankOwnershipChanged,
    DebitNotAuthorized,
    IncorrectAccountHolderAddress,
    IncorrectAccountHolderName,
    IncorrectAccountHolderTaxId,
    InsufficientFunds,
    InvalidAccountNumber,
    InvalidCurrency,
    NoAccount,
    Other,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MandatePaymentMethodDetails {
    pub acss_debit: Option<MandateAcssDebit>,
    pub au_becs_debit: Option<MandateAuBecsDebit>,
    pub bacs_debit: Option<MandateBacsDebit>,
    pub card: Option<CardMandatePaymentMethodDetails>,
    pub cashapp: Option<MandateCashapp>,
    pub link: Option<MandateLink>,
    pub paypal: Option<MandatePaypal>,
    pub sepa_debit: Option<MandateSepaDebit>,
    pub r#type: String,
    pub us_bank_account: Option<MandateUsBankAccount>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PriceTiersMode {
    Graduated,
    Volume,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetupAttemptPaymentMethodDetailsCardWallet {
    pub apple_pay: Option<PaymentMethodDetailsCardWalletApplePay>,
    pub google_pay: Option<PaymentMethodDetailsCardWalletGooglePay>,
    pub r#type: SetupAttemptPaymentMethodDetailsCardWalletType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionCustomText {
    pub after_submit: ReplaceMeWithAnyOfSpec,
    pub shipping_address: ReplaceMeWithAnyOfSpec,
    pub submit: ReplaceMeWithAnyOfSpec,
    pub terms_of_service_acceptance: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PortalFlowsFlowAfterCompletion {
    pub hosted_confirmation: ReplaceMeWithAnyOfSpec,
    pub redirect: ReplaceMeWithAnyOfSpec,
    pub r#type: PortalFlowsFlowAfterCompletionType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsOxxo {
    pub number: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardholderSpendingLimitCategoriesItem {
    AcRefrigerationRepair,
    AccountingBookkeepingServices,
    AdvertisingServices,
    AgriculturalCooperative,
    AirlinesAirCarriers,
    AirportsFlyingFields,
    AmbulanceServices,
    AmusementParksCarnivals,
    AntiqueReproductions,
    AntiqueShops,
    Aquariums,
    ArchitecturalSurveyingServices,
    ArtDealersAndGalleries,
    ArtistsSupplyAndCraftShops,
    AutoAndHomeSupplyStores,
    AutoBodyRepairShops,
    AutoPaintShops,
    AutoServiceShops,
    AutomatedCashDisburse,
    AutomatedFuelDispensers,
    AutomobileAssociations,
    AutomotivePartsAndAccessoriesStores,
    AutomotiveTireStores,
    BailAndBondPayments,
    Bakeries,
    BandsOrchestras,
    BarberAndBeautyShops,
    BettingCasinoGambling,
    BicycleShops,
    BilliardPoolEstablishments,
    BoatDealers,
    BoatRentalsAndLeases,
    BookStores,
    BooksPeriodicalsAndNewspapers,
    BowlingAlleys,
    BusLines,
    BusinessSecretarialSchools,
    BuyingShoppingServices,
    CableSatelliteAndOtherPayTelevisionAndRadio,
    CameraAndPhotographicSupplyStores,
    CandyNutAndConfectioneryStores,
    CarAndTruckDealersNewUsed,
    CarAndTruckDealersUsedOnly,
    CarRentalAgencies,
    CarWashes,
    CarpentryServices,
    CarpetUpholsteryCleaning,
    Caterers,
    CharitableAndSocialServiceOrganizationsFundraising,
    ChemicalsAndAlliedProducts,
    ChildCareServices,
    ChildrensAndInfantsWearStores,
    ChiropodistsPodiatrists,
    Chiropractors,
    CigarStoresAndStands,
    CivicSocialFraternalAssociations,
    CleaningAndMaintenance,
    ClothingRental,
    CollegesUniversities,
    CommercialEquipment,
    CommercialFootwear,
    CommercialPhotographyArtAndGraphics,
    CommuterTransportAndFerries,
    ComputerNetworkServices,
    ComputerProgramming,
    ComputerRepair,
    ComputerSoftwareStores,
    ComputersPeripheralsAndSoftware,
    ConcreteWorkServices,
    ConstructionMaterials,
    ConsultingPublicRelations,
    CorrespondenceSchools,
    CosmeticStores,
    CounselingServices,
    CountryClubs,
    CourierServices,
    CourtCosts,
    CreditReportingAgencies,
    CruiseLines,
    DairyProductsStores,
    DanceHallStudiosSchools,
    DatingEscortServices,
    DentistsOrthodontists,
    DepartmentStores,
    DetectiveAgencies,
    DigitalGoodsApplications,
    DigitalGoodsGames,
    DigitalGoodsLargeVolume,
    DigitalGoodsMedia,
    DirectMarketingCatalogMerchant,
    DirectMarketingCombinationCatalogAndRetailMerchant,
    DirectMarketingInboundTelemarketing,
    DirectMarketingInsuranceServices,
    DirectMarketingOther,
    DirectMarketingOutboundTelemarketing,
    DirectMarketingSubscription,
    DirectMarketingTravel,
    DiscountStores,
    Doctors,
    DoorToDoorSales,
    DraperyWindowCoveringAndUpholsteryStores,
    DrinkingPlaces,
    DrugStoresAndPharmacies,
    DrugsDrugProprietariesAndDruggistSundries,
    DryCleaners,
    DurableGoods,
    DutyFreeStores,
    EatingPlacesRestaurants,
    EducationalServices,
    ElectricRazorStores,
    ElectricVehicleCharging,
    ElectricalPartsAndEquipment,
    ElectricalServices,
    ElectronicsRepairShops,
    ElectronicsStores,
    ElementarySecondarySchools,
    EmergencyServicesGcasVisaUseOnly,
    EmploymentTempAgencies,
    EquipmentRental,
    ExterminatingServices,
    FamilyClothingStores,
    FastFoodRestaurants,
    FinancialInstitutions,
    FinesGovernmentAdministrativeEntities,
    FireplaceFireplaceScreensAndAccessoriesStores,
    FloorCoveringStores,
    Florists,
    FloristsSuppliesNurseryStockAndFlowers,
    FreezerAndLockerMeatProvisioners,
    FuelDealersNonAutomotive,
    FuneralServicesCrematories,
    FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances,
    FurnitureRepairRefinishing,
    FurriersAndFurShops,
    GeneralServices,
    GiftCardNoveltyAndSouvenirShops,
    GlassPaintAndWallpaperStores,
    GlasswareCrystalStores,
    GolfCoursesPublic,
    GovernmentLicensedHorseDogRacingUsRegionOnly,
    GovernmentLicensedOnlineCasionsOnlineGamblingUsRegionOnly,
    GovernmentOwnedLotteriesNonUsRegion,
    GovernmentOwnedLotteriesUsRegionOnly,
    GovernmentServices,
    GroceryStoresSupermarkets,
    HardwareEquipmentAndSupplies,
    HardwareStores,
    HealthAndBeautySpas,
    HearingAidsSalesAndSupplies,
    HeatingPlumbingAC,
    HobbyToyAndGameShops,
    HomeSupplyWarehouseStores,
    Hospitals,
    HotelsMotelsAndResorts,
    HouseholdApplianceStores,
    IndustrialSupplies,
    InformationRetrievalServices,
    InsuranceDefault,
    InsuranceUnderwritingPremiums,
    IntraCompanyPurchases,
    JewelryStoresWatchesClocksAndSilverwareStores,
    LandscapingServices,
    Laundries,
    LaundryCleaningServices,
    LegalServicesAttorneys,
    LuggageAndLeatherGoodsStores,
    LumberBuildingMaterialsStores,
    ManualCashDisburse,
    MarinasServiceAndSupplies,
    Marketplaces,
    MasonryStoneworkAndPlaster,
    MassageParlors,
    MedicalAndDentalLabs,
    MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies,
    MedicalServices,
    MembershipOrganizations,
    MensAndBoysClothingAndAccessoriesStores,
    MensWomensClothingStores,
    MetalServiceCenters,
    Miscellaneous,
    MiscellaneousApparelAndAccessoryShops,
    MiscellaneousAutoDealers,
    MiscellaneousBusinessServices,
    MiscellaneousFoodStores,
    MiscellaneousGeneralMerchandise,
    MiscellaneousGeneralServices,
    MiscellaneousHomeFurnishingSpecialtyStores,
    MiscellaneousPublishingAndPrinting,
    MiscellaneousRecreationServices,
    MiscellaneousRepairShops,
    MiscellaneousSpecialtyRetail,
    MobileHomeDealers,
    MotionPictureTheaters,
    MotorFreightCarriersAndTrucking,
    MotorHomesDealers,
    MotorVehicleSuppliesAndNewParts,
    MotorcycleShopsAndDealers,
    MotorcycleShopsDealers,
    MusicStoresMusicalInstrumentsPianosAndSheetMusic,
    NewsDealersAndNewsstands,
    NonFiMoneyOrders,
    NonFiStoredValueCardPurchaseLoad,
    NondurableGoods,
    NurseriesLawnAndGardenSupplyStores,
    NursingPersonalCare,
    OfficeAndCommercialFurniture,
    OpticiansEyeglasses,
    OptometristsOphthalmologist,
    OrthopedicGoodsProstheticDevices,
    Osteopaths,
    PackageStoresBeerWineAndLiquor,
    PaintsVarnishesAndSupplies,
    ParkingLotsGarages,
    PassengerRailways,
    PawnShops,
    PetShopsPetFoodAndSupplies,
    PetroleumAndPetroleumProducts,
    PhotoDeveloping,
    PhotographicPhotocopyMicrofilmEquipmentAndSupplies,
    PhotographicStudios,
    PictureVideoProduction,
    PieceGoodsNotionsAndOtherDryGoods,
    PlumbingHeatingEquipmentAndSupplies,
    PoliticalOrganizations,
    PostalServicesGovernmentOnly,
    PreciousStonesAndMetalsWatchesAndJewelry,
    ProfessionalServices,
    PublicWarehousingAndStorage,
    QuickCopyReproAndBlueprint,
    Railroads,
    RealEstateAgentsAndManagersRentals,
    RecordStores,
    RecreationalVehicleRentals,
    ReligiousGoodsStores,
    ReligiousOrganizations,
    RoofingSidingSheetMetal,
    SecretarialSupportServices,
    SecurityBrokersDealers,
    ServiceStations,
    SewingNeedleworkFabricAndPieceGoodsStores,
    ShoeRepairHatCleaning,
    ShoeStores,
    SmallApplianceRepair,
    SnowmobileDealers,
    SpecialTradeServices,
    SpecialtyCleaning,
    SportingGoodsStores,
    SportingRecreationCamps,
    SportsAndRidingApparelStores,
    SportsClubsFields,
    StampAndCoinStores,
    StationaryOfficeSuppliesPrintingAndWritingPaper,
    StationeryStoresOfficeAndSchoolSupplyStores,
    SwimmingPoolsSales,
    TUiTravelGermany,
    TailorsAlterations,
    TaxPaymentsGovernmentAgencies,
    TaxPreparationServices,
    TaxicabsLimousines,
    TelecommunicationEquipmentAndTelephoneSales,
    TelecommunicationServices,
    TelegraphServices,
    TentAndAwningShops,
    TestingLaboratories,
    TheatricalTicketAgencies,
    Timeshares,
    TireRetreadingAndRepair,
    TollsBridgeFees,
    TouristAttractionsAndExhibits,
    TowingServices,
    TrailerParksCampgrounds,
    TransportationServices,
    TravelAgenciesTourOperators,
    TruckStopIteration,
    TruckUtilityTrailerRentals,
    TypesettingPlateMakingAndRelatedServices,
    TypewriterStores,
    USFederalGovernmentAgenciesOrDepartments,
    UniformsCommercialClothing,
    UsedMerchandiseAndSecondhandStores,
    Utilities,
    VarietyStores,
    VeterinaryServices,
    VideoAmusementGameSupplies,
    VideoGameArcades,
    VideoTapeRentalStores,
    VocationalTradeSchools,
    WatchJewelryRepair,
    WeldingRepair,
    WholesaleClubs,
    WigAndToupeeStores,
    WiresMoneyOrders,
    WomensAccessoryAndSpecialtyShops,
    WomensReadyToWearStores,
    WreckingAndSalvageYards,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OutboundPaymentsPaymentMethodDetails {
    pub billing_details: TreasurySharedResourceBillingDetails,
    pub financial_account: Option<OutboundPaymentsPaymentMethodDetailsFinancialAccount>,
    pub r#type: OutboundPaymentsPaymentMethodDetailsType,
    pub us_bank_account: Option<OutboundPaymentsPaymentMethodDetailsUsBankAccount>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SourceOwner {
    pub address: ReplaceMeWithAnyOfSpec,
    pub email: String,
    pub name: String,
    pub phone: String,
    pub verified_address: ReplaceMeWithAnyOfSpec,
    pub verified_email: String,
    pub verified_name: String,
    pub verified_phone: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentityVerificationSessionStatus {
    Canceled,
    Processing,
    RequiresInput,
    Verified,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Topup {
    pub amount: i64,
    pub balance_transaction: ReplaceMeWithAnyOfSpec,
    pub created: i64,
    pub currency: String,
    pub description: String,
    pub expected_availability_date: i64,
    pub failure_code: String,
    pub failure_message: String,
    pub id: String,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub object: String,
    pub source: ReplaceMeWithAnyOfSpec,
    pub statement_descriptor: String,
    pub status: TopupStatus,
    pub transfer_group: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryDebitReversalNetwork {
    Ach,
    Card,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodZip {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingTransactionFlightDataLeg {
    pub arrival_airport_code: String,
    pub carrier: String,
    pub departure_airport_code: String,
    pub flight_number: String,
    pub service_class: String,
    pub stopover_allowed: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    Business,
    Personal,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsFpxAccountHolderType {
    Company,
    Individual,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodUsBankAccountBlockedNetworkCode {
    R02,
    R03,
    R04,
    R05,
    R07,
    R08,
    R10,
    R11,
    R16,
    R20,
    R29,
    R31,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CheckoutCustomerBalancePaymentMethodOptions {
    pub bank_transfer: Option<CheckoutCustomerBalanceBankTransferPaymentMethodOptions>,
    pub funding_type: CheckoutCustomerBalancePaymentMethodOptionsFundingType,
    pub setup_future_usage: Option<
        CheckoutCustomerBalancePaymentMethodOptionsSetupFutureUsage,
    >,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub address: Option<ReplaceMeWithAnyOfSpec>,
    pub balance: Option<i64>,
    pub cash_balance: Option<ReplaceMeWithAnyOfSpec>,
    pub created: i64,
    pub currency: Option<String>,
    pub default_source: ReplaceMeWithAnyOfSpec,
    pub delinquent: Option<bool>,
    pub description: String,
    pub discount: Option<ReplaceMeWithAnyOfSpec>,
    pub email: String,
    pub id: String,
    pub invoice_credit_balance: Option<CustomerInvoiceCreditBalance>,
    pub invoice_prefix: Option<String>,
    pub invoice_settings: Option<InvoiceSettingCustomerSetting>,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub name: Option<String>,
    pub next_invoice_sequence: Option<i64>,
    pub object: String,
    pub phone: Option<String>,
    pub preferred_locales: Option<Vec<String>>,
    pub shipping: ReplaceMeWithAnyOfSpec,
    pub sources: Option<CustomerSources>,
    pub subscriptions: Option<CustomerSubscriptions>,
    pub tax: Option<CustomerTax>,
    pub tax_exempt: Option<CustomerTaxExempt>,
    pub tax_ids: Option<CustomerTaxIds>,
    pub test_clock: Option<ReplaceMeWithAnyOfSpec>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodFpxAccountHolderType {
    Company,
    Individual,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodOptionsGrabpay {
    pub setup_future_usage: Option<PaymentMethodOptionsGrabpaySetupFutureUsage>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetupAttemptPaymentMethodDetailsCard {
    pub brand: String,
    pub checks: ReplaceMeWithAnyOfSpec,
    pub country: String,
    pub description: Option<String>,
    pub exp_month: i64,
    pub exp_year: i64,
    pub fingerprint: Option<String>,
    pub funding: String,
    pub iin: Option<String>,
    pub issuer: Option<String>,
    pub last4: String,
    pub network: String,
    pub three_d_secure: ReplaceMeWithAnyOfSpec,
    pub wallet: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxProductRegistrationsResourceCountryOptionsUsLocalAmusementTax {
    pub jurisdiction: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentPaymentMethodOptionsBlikSetupFutureUsage {
    None,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodCardWalletSamsungPay {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PayoutReconciliationStatus {
    Completed,
    InProgress,
    NotApplicable,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeletedRadarValueListItem {
    pub deleted: bool,
    pub id: String,
    pub object: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentLinksResourceCustomFields {
    pub dropdown: Option<PaymentLinksResourceCustomFieldsDropdown>,
    pub key: String,
    pub label: PaymentLinksResourceCustomFieldsLabel,
    pub numeric: Option<PaymentLinksResourceCustomFieldsNumeric>,
    pub optional: bool,
    pub text: Option<PaymentLinksResourceCustomFieldsText>,
    pub r#type: PaymentLinksResourceCustomFieldsType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsPaynowSetupFutureUsage {
    None,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance {
    Payments,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutSessionLocale {
    Auto,
    Bg,
    Cs,
    Da,
    De,
    El,
    En,
    EnGB,
    Es,
    Es419,
    Et,
    Fi,
    Fil,
    Fr,
    FrCA,
    Hr,
    Hu,
    Id,
    It,
    Ja,
    Ko,
    Lt,
    Lv,
    Ms,
    Mt,
    Nb,
    Nl,
    Pl,
    Pt,
    PtBR,
    Ro,
    Ru,
    Sk,
    Sl,
    Sv,
    Th,
    Tr,
    Vi,
    Zh,
    ZhHK,
    ZhTW,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionSchedulesResourceDefaultSettingsCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Application {
    pub id: String,
    pub name: String,
    pub object: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingCardSpendingLimit {
    pub amount: i64,
    pub categories: Vec<IssuingCardSpendingLimitCategoriesItem>,
    pub interval: IssuingCardSpendingLimitInterval,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsCardWalletLink {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsCardMandateOptionsInterval {
    Day,
    Month,
    Sporadic,
    Week,
    Year,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryReceivedCreditFailureCode {
    AccountClosed,
    AccountFrozen,
    Other,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsKonbini {
    pub store: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodPix {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SubscriptionSchedulePhaseConfiguration {
    pub add_invoice_items: Vec<SubscriptionScheduleAddInvoiceItem>,
    pub application_fee_percent: f64,
    pub automatic_tax: Option<SchedulesPhaseAutomaticTax>,
    pub billing_cycle_anchor: SubscriptionSchedulePhaseConfigurationBillingCycleAnchor,
    pub billing_thresholds: ReplaceMeWithAnyOfSpec,
    pub collection_method: SubscriptionSchedulePhaseConfigurationCollectionMethod,
    pub coupon: ReplaceMeWithAnyOfSpec,
    pub currency: String,
    pub default_payment_method: ReplaceMeWithAnyOfSpec,
    pub default_tax_rates: Option<Vec<TaxRate>>,
    pub description: String,
    pub end_date: i64,
    pub invoice_settings: ReplaceMeWithAnyOfSpec,
    pub items: Vec<SubscriptionScheduleConfigurationItem>,
    pub metadata: HashMap<String, String>,
    pub on_behalf_of: ReplaceMeWithAnyOfSpec,
    pub proration_behavior: SubscriptionSchedulePhaseConfigurationProrationBehavior,
    pub start_date: i64,
    pub transfer_data: ReplaceMeWithAnyOfSpec,
    pub trial_end: i64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BankConnectionsResourceBalanceRefreshStatus {
    Failed,
    Pending,
    Succeeded,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution {
    ContactStripe,
    ProvideInformation,
    RemoveRestriction,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodOptionsKonbini {
    pub confirmation_number: String,
    pub expires_after_days: i64,
    pub expires_at: i64,
    pub product_description: String,
    pub setup_future_usage: Option<PaymentMethodOptionsKonbiniSetupFutureUsage>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CreditNoteTaxAmountTaxabilityReason {
    CustomerExempt,
    NotCollecting,
    NotSubjectToTax,
    NotSupported,
    PortionProductExempt,
    PortionReducedRated,
    PortionStandardRated,
    ProductExempt,
    ProductExemptHoliday,
    ProportionallyRated,
    ReducedRated,
    ReverseCharge,
    StandardRated,
    TaxableBasisReduced,
    ZeroRated,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetupIntentPaymentMethodOptionsCard {
    pub mandate_options: ReplaceMeWithAnyOfSpec,
    pub network: SetupIntentPaymentMethodOptionsCardNetwork,
    pub request_three_d_secure: SetupIntentPaymentMethodOptionsCardRequestThreeDSecure,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PersonFutureRequirements {
    pub alternatives: Vec<AccountRequirementsAlternative>,
    pub currently_due: Vec<String>,
    pub errors: Vec<AccountRequirementsError>,
    pub eventually_due: Vec<String>,
    pub past_due: Vec<String>,
    pub pending_verification: Vec<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxProductResourceTaxTransactionResourceReversal {
    pub original_transaction: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestHelpersTestClockStatus {
    Advancing,
    InternalFailure,
    Ready,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutAfterpayClearpayPaymentMethodOptionsSetupFutureUsage {
    None,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsSofortPreferredLanguage {
    De,
    En,
    Es,
    Fr,
    It,
    Nl,
    Pl,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetupIntentPaymentMethodOptionsLink {
    pub persistent_token: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BankConnectionsResourceBalanceApiResourceCreditBalanceUsed {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsCardInstallments {
    pub plan: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork {
    Stripe,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesRevolutPayPayments {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GelatoDataIdNumberReportDate {
    pub day: i64,
    pub month: i64,
    pub year: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsP24 {
    pub bank: PaymentMethodDetailsP24Bank,
    pub reference: String,
    pub verified_name: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FinancialReportingFinanceReportRunRunParameters {
    pub columns: Option<Vec<String>>,
    pub connected_account: Option<String>,
    pub currency: Option<String>,
    pub interval_end: Option<i64>,
    pub interval_start: Option<i64>,
    pub payout: Option<String>,
    pub reporting_category: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PortalFeatures {
    pub customer_update: PortalCustomerUpdate,
    pub invoice_history: PortalInvoiceList,
    pub payment_method_update: PortalPaymentMethodUpdate,
    pub subscription_cancel: PortalSubscriptionCancel,
    pub subscription_pause: PortalSubscriptionPause,
    pub subscription_update: PortalSubscriptionUpdate,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RefundDestinationDetailsCard {
    pub reference: Option<String>,
    pub reference_status: Option<String>,
    pub reference_type: Option<String>,
    pub r#type: RefundDestinationDetailsCardType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoicePaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
    Challenge,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetupAttemptPaymentMethodDetailsAuBecsDebit {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodOptionsCustomerBalance {
    pub bank_transfer: Option<PaymentMethodOptionsCustomerBalanceBankTransfer>,
    pub funding_type: PaymentMethodOptionsCustomerBalanceFundingType,
    pub setup_future_usage: Option<PaymentMethodOptionsCustomerBalanceSetupFutureUsage>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetupAttemptPaymentMethodDetailsBacsDebit {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SourceTypeEps {
    pub reference: Option<String>,
    pub statement_descriptor: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentLinksResourceCustomFieldsDropdown {
    pub options: Vec<PaymentLinksResourceCustomFieldsDropdownOption>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodConfigResourceDisplayPreferenceValue {
    Off,
    On,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaxIdVerificationStatus {
    Pending,
    Unavailable,
    Unverified,
    Verified,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxProductRegistrationsResourceCountryOptionsCanada {
    pub province_standard: Option<
        TaxProductRegistrationsResourceCountryOptionsCaProvinceStandard,
    >,
    pub r#type: TaxProductRegistrationsResourceCountryOptionsCanadaType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceivedPaymentMethodDetailsFinancialAccountNetwork {
    Stripe,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingTransactionLodgingData {
    pub check_in_at: i64,
    pub nights: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodConfigBizPaymentMethodConfigurationDetails {
    pub id: String,
    pub parent: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryTransactionEntryFlowType {
    CreditReversal,
    DebitReversal,
    InboundTransfer,
    IssuingAuthorization,
    Other,
    OutboundPayment,
    OutboundTransfer,
    ReceivedCredit,
    ReceivedDebit,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryReceivedDebit {
    pub amount: i64,
    pub created: i64,
    pub currency: String,
    pub description: String,
    pub failure_code: TreasuryReceivedDebitFailureCode,
    pub financial_account: String,
    pub hosted_regulatory_receipt_url: String,
    pub id: String,
    pub initiating_payment_method_details: Option<
        TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails,
    >,
    pub linked_flows: TreasuryReceivedDebitsResourceLinkedFlows,
    pub livemode: bool,
    pub network: TreasuryReceivedDebitNetwork,
    pub object: String,
    pub reversal_details: ReplaceMeWithAnyOfSpec,
    pub status: TreasuryReceivedDebitStatus,
    pub transaction: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AccountCapabilityRequirements {
    pub alternatives: Vec<AccountRequirementsAlternative>,
    pub current_deadline: i64,
    pub currently_due: Vec<String>,
    pub disabled_reason: String,
    pub errors: Vec<AccountRequirementsError>,
    pub eventually_due: Vec<String>,
    pub past_due: Vec<String>,
    pub pending_verification: Vec<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsCardChecks {
    pub address_line1_check: String,
    pub address_postal_code_check: String,
    pub cvc_check: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanInterval {
    Day,
    Month,
    Week,
    Year,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Shipping {
    pub address: Option<Address>,
    pub carrier: Option<String>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub tracking_number: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodIdeal {
    pub bank: PaymentMethodIdealBank,
    pub bic: PaymentMethodIdealBic,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryTransactionEntries {
    pub data: Vec<TreasuryTransactionEntry>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryInboundTransferStatus {
    Canceled,
    Failed,
    Processing,
    Succeeded,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentPaymentMethodOptionsUsBankAccountSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryTransactionStatus {
    Open,
    Posted,
    Void,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductResourceCustomerDetailsResourceTaxIdType {
    AdNrt,
    AeTrn,
    ArCuit,
    AuAbn,
    AuArn,
    BgUic,
    BoTin,
    BrCnpj,
    BrCpf,
    CaBn,
    CaGstHst,
    CaPstBc,
    CaPstMb,
    CaPstSk,
    CaQst,
    ChVat,
    ClTin,
    CnTin,
    CoNit,
    CrTin,
    DoRcn,
    EcRuc,
    EgTin,
    EsCif,
    EuOssVat,
    EuVat,
    GbVat,
    GeVat,
    HkBr,
    HuTin,
    IdNpwp,
    IlVat,
    InGst,
    IsVat,
    JpCn,
    JpRn,
    JpTrn,
    KePin,
    KrBrn,
    LiUid,
    MxRfc,
    MyFrp,
    MyItn,
    MySst,
    NoVat,
    NzGst,
    PeRuc,
    PhTin,
    RoTin,
    RsPib,
    RuInn,
    RuKpp,
    SaVat,
    SgGst,
    SgUen,
    SiTin,
    SvNit,
    ThVat,
    TrTin,
    TwVat,
    UaVat,
    Unknown,
    UsEin,
    UyRuc,
    VeRif,
    VnTin,
    ZaVat,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SubscriptionItems {
    pub data: Vec<SubscriptionItem>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodOptionsZip {
    pub setup_future_usage: Option<PaymentMethodOptionsZipSetupFutureUsage>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryReceivedCreditsResourceReversalDetails {
    pub deadline: i64,
    pub restricted_reason: TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesGrabpayPayments {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryCreditReversalStatus {
    Canceled,
    Posted,
    Processing,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypesItem {
    India,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InvoicePaymentMethodOptionsBancontact {
    pub preferred_language: InvoicePaymentMethodOptionsBancontactPreferredLanguage,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodPromptpay {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PortalFlowsFlowSubscriptionCancel {
    pub retention: ReplaceMeWithAnyOfSpec,
    pub subscription: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesPaynowPayments {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingAuthorizationNetworkData {
    pub acquiring_institution_id: String,
    pub system_trace_audit_number: String,
    pub transaction_id: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingAuthorizationRequest {
    pub amount: i64,
    pub amount_details: ReplaceMeWithAnyOfSpec,
    pub approved: bool,
    pub authorization_code: String,
    pub created: i64,
    pub currency: String,
    pub merchant_amount: i64,
    pub merchant_currency: String,
    pub network_risk_score: i64,
    pub reason: IssuingAuthorizationRequestReason,
    pub reason_message: String,
    pub requested_at: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodOxxo {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ConnectEmbeddedAccountSessionCreateComponents {
    pub account_onboarding: ConnectEmbeddedBaseConfigClaim,
    pub payment_details: ConnectEmbeddedPaymentsConfig,
    pub payments: ConnectEmbeddedPaymentsConfig,
    pub payouts: ConnectEmbeddedPayoutsConfig,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MandateType {
    MultiUse,
    SingleUse,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Refund {
    pub amount: i64,
    pub balance_transaction: ReplaceMeWithAnyOfSpec,
    pub charge: ReplaceMeWithAnyOfSpec,
    pub created: i64,
    pub currency: String,
    pub description: Option<String>,
    pub destination_details: Option<RefundDestinationDetails>,
    pub failure_balance_transaction: Option<ReplaceMeWithAnyOfSpec>,
    pub failure_reason: Option<String>,
    pub id: String,
    pub instructions_email: Option<String>,
    pub metadata: HashMap<String, String>,
    pub next_action: Option<RefundNextAction>,
    pub object: String,
    pub payment_intent: ReplaceMeWithAnyOfSpec,
    pub reason: RefundReason,
    pub receipt_number: String,
    pub source_transfer_reversal: ReplaceMeWithAnyOfSpec,
    pub status: String,
    pub transfer_reversal: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxProductRegistrationsResourceCountryOptionsSimplified {
    pub r#type: TaxProductRegistrationsResourceCountryOptionsSimplifiedType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoicePaymentMethodOptionsBancontactPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PortalFlowsAfterCompletionRedirect {
    pub return_url: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClimateOrderCancellationReason {
    Expired,
    ProductUnavailable,
    Requested,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
    UsBankTransfer,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeletedTerminalReader {
    pub deleted: bool,
    pub id: String,
    pub object: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinancialConnectionsAccountStatus {
    Active,
    Disconnected,
    Inactive,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentNextActionKonbiniStores {
    pub familymart: ReplaceMeWithAnyOfSpec,
    pub lawson: ReplaceMeWithAnyOfSpec,
    pub ministop: ReplaceMeWithAnyOfSpec,
    pub seicomart: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDomainResourcePaymentMethodStatusStatus {
    Active,
    Inactive,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThreeDSecureDetailsResult {
    AttemptAcknowledged,
    Authenticated,
    Exempted,
    Failed,
    NotSupported,
    ProcessingError,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsAffirm {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NotificationEventDataPreviousAttributes {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountsResourceAchToggleSettingsStatus {
    Active,
    Pending,
    Restricted,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GelatoDataDocumentReportIssuedDate {
    pub day: i64,
    pub month: i64,
    pub year: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SchedulesPhaseAutomaticTax {
    pub enabled: bool,
    pub liability: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoicePaymentMethodOptionsUsBankAccountVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceEuBankTransfer {
    pub bic: String,
    pub iban_last4: String,
    pub sender_name: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CheckoutGrabPayPaymentMethodOptions {
    pub setup_future_usage: Option<CheckoutGrabPayPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SubscriptionDetailsData {
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionCustomFieldsOption {
    pub label: String,
    pub value: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MandateAcssDebitTransactionType {
    Business,
    Personal,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct QuotesResourceStatusTransitions {
    pub accepted_at: i64,
    pub canceled_at: i64,
    pub finalized_at: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InvoicePaymentMethodOptionsCustomerBalanceBankTransfer {
    pub eu_bank_transfer: Option<
        InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer,
    >,
    pub r#type: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypesItem {
    Aba,
    Iban,
    Sepa,
    SortCode,
    Spei,
    Swift,
    Zengin,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingDispute {
    pub amount: i64,
    pub balance_transactions: Vec<BalanceTransaction>,
    pub created: i64,
    pub currency: String,
    pub evidence: IssuingDisputeEvidence,
    pub id: String,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub object: String,
    pub status: IssuingDisputeStatus,
    pub transaction: ReplaceMeWithAnyOfSpec,
    pub treasury: Option<ReplaceMeWithAnyOfSpec>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaxTransactionType {
    Reversal,
    Transaction,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentFlowsPrivatePaymentMethodsAlipayDetails {
    pub buyer_id: Option<String>,
    pub fingerprint: String,
    pub transaction_id: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShippingRateType {
    FixedAmount,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesLegacyPayments {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CancellationDetails {
    pub comment: String,
    pub feedback: CancellationDetailsFeedback,
    pub reason: CancellationDetailsReason,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CheckoutSwishPaymentMethodOptions {
    pub reference: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BankConnectionsResourceOwnershipRefresh {
    pub last_attempted_at: i64,
    pub status: BankConnectionsResourceOwnershipRefreshStatus,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NotificationEventData {
    pub object: NotificationEventDataObject,
    pub previous_attributes: Option<NotificationEventDataPreviousAttributes>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType {
    Checking,
    Savings,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentLinksResourcePhoneNumberCollection {
    pub enabled: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct QuotesResourceSubscriptionDataSubscriptionData {
    pub description: String,
    pub effective_date: i64,
    pub metadata: HashMap<String, String>,
    pub trial_period_days: i64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxIDsOwner {
    pub account: Option<ReplaceMeWithAnyOfSpec>,
    pub application: Option<ReplaceMeWithAnyOfSpec>,
    pub customer: Option<ReplaceMeWithAnyOfSpec>,
    pub r#type: TaxIDsOwnerType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxProductResourceLineItemTaxBreakdown {
    pub amount: i64,
    pub jurisdiction: TaxProductResourceJurisdiction,
    pub sourcing: TaxProductResourceLineItemTaxBreakdownSourcing,
    pub tax_rate_details: ReplaceMeWithAnyOfSpec,
    pub taxability_reason: TaxProductResourceLineItemTaxBreakdownTaxabilityReason,
    pub taxable_amount: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PriceCurrencyOptions {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryInboundTransfersResourceFailureDetails {
    pub code: TreasuryInboundTransfersResourceFailureDetailsCode,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FundingInstructionsFundingType {
    BankTransfer,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingDisputeNotReceivedEvidence {
    pub additional_documentation: ReplaceMeWithAnyOfSpec,
    pub expected_at: i64,
    pub explanation: String,
    pub product_description: String,
    pub product_type: IssuingDisputeNotReceivedEvidenceProductType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GelatoSelfieReportError {
    pub code: GelatoSelfieReportErrorCode,
    pub reason: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RadarValueList {
    pub alias: String,
    pub created: i64,
    pub created_by: String,
    pub id: String,
    pub item_type: RadarValueListItemType,
    pub list_items: RadarValueListListItems,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub name: String,
    pub object: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SourceTypeSepaDebit {
    pub bank_code: Option<String>,
    pub branch_code: Option<String>,
    pub country: Option<String>,
    pub fingerprint: Option<String>,
    pub last4: Option<String>,
    pub mandate_reference: Option<String>,
    pub mandate_url: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryFinancialAccountsResourceInboundTransfers {
    pub ach: Option<TreasuryFinancialAccountsResourceAchToggleSettings>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetupAttempt {
    pub application: ReplaceMeWithAnyOfSpec,
    pub attach_to_self: Option<bool>,
    pub created: i64,
    pub customer: ReplaceMeWithAnyOfSpec,
    pub flow_directions: Vec<SetupAttemptFlowDirectionsItem>,
    pub id: String,
    pub livemode: bool,
    pub object: String,
    pub on_behalf_of: ReplaceMeWithAnyOfSpec,
    pub payment_method: ReplaceMeWithAnyOfSpec,
    pub payment_method_details: SetupAttemptPaymentMethodDetails,
    pub setup_error: ReplaceMeWithAnyOfSpec,
    pub setup_intent: ReplaceMeWithAnyOfSpec,
    pub status: String,
    pub usage: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InvoicesStatusTransitions {
    pub finalized_at: i64,
    pub marked_uncollectible_at: i64,
    pub paid_at: i64,
    pub voided_at: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingAuthorizationThreeDSecure {
    pub result: IssuingAuthorizationThreeDSecureResult,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodAcssDebit {
    pub bank_name: String,
    pub fingerprint: String,
    pub institution_number: String,
    pub last4: String,
    pub transit_number: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsCardMandateOptionsAmountType {
    Fixed,
    Maximum,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SourceTransactionAchCreditTransferData {
    pub customer_data: Option<String>,
    pub fingerprint: Option<String>,
    pub last4: Option<String>,
    pub routing_number: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodKonbini {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutboundTransfersPaymentMethodDetailsType {
    UsBankAccount,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CheckoutPaynowPaymentMethodOptions {
    pub setup_future_usage: Option<CheckoutPaynowPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    Ach,
    UsDomesticWire,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentPaymentMethodOptionsMandateOptionsSepaDebit {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductResourceCustomerDetailsAddressSource {
    Billing,
    Shipping,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentPaymentMethodOptionsCardCaptureMethod {
    Manual,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsRevolutPay {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodIdealBic {
    ABNANL2A,
    ASNBNL21,
    BITSNL2A,
    BUNQNL2A,
    FVLBNL22,
    HANDNL2A,
    INGBNL2A,
    KNABNL2H,
    MOYONL21,
    NNBANL2G,
    NTSBDEB1,
    RABONL2U,
    RBRBNL21,
    REVOIE23,
    REVOLT21,
    SNSBNL2A,
    TRIONL2U,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CheckoutSepaDebitPaymentMethodOptions {
    pub setup_future_usage: Option<
        CheckoutSepaDebitPaymentMethodOptionsSetupFutureUsage,
    >,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutEpsPaymentMethodOptionsSetupFutureUsage {
    None,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingDisputeCanceledEvidenceReturnStatus {
    MerchantRejected,
    Successful,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OutboundPaymentsPaymentMethodDetailsUsBankAccount {
    pub account_holder_type: OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType,
    pub account_type: OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountType,
    pub bank_name: String,
    pub fingerprint: String,
    pub last4: String,
    pub network: OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork,
    pub routing_number: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountPendingFeaturesItem {
    CardIssuing,
    DepositInsurance,
    FinancialAddressesAba,
    InboundTransfersAch,
    IntraStripeFlows,
    OutboundPaymentsAch,
    OutboundPaymentsUsDomesticWire,
    OutboundTransfersAch,
    OutboundTransfersUsDomesticWire,
    RemoteDepositCapture,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SourceTypeKlarna {
    pub background_image_url: Option<String>,
    pub client_token: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub locale: Option<String>,
    pub logo_url: Option<String>,
    pub page_title: Option<String>,
    pub pay_later_asset_urls_descriptive: Option<String>,
    pub pay_later_asset_urls_standard: Option<String>,
    pub pay_later_name: Option<String>,
    pub pay_later_redirect_url: Option<String>,
    pub pay_now_asset_urls_descriptive: Option<String>,
    pub pay_now_asset_urls_standard: Option<String>,
    pub pay_now_name: Option<String>,
    pub pay_now_redirect_url: Option<String>,
    pub pay_over_time_asset_urls_descriptive: Option<String>,
    pub pay_over_time_asset_urls_standard: Option<String>,
    pub pay_over_time_name: Option<String>,
    pub pay_over_time_redirect_url: Option<String>,
    pub payment_method_categories: Option<String>,
    pub purchase_country: Option<String>,
    pub purchase_type: Option<String>,
    pub redirect_url: Option<String>,
    pub shipping_delay: Option<i64>,
    pub shipping_first_name: Option<String>,
    pub shipping_last_name: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ApplicationFee {
    pub account: ReplaceMeWithAnyOfSpec,
    pub amount: i64,
    pub amount_refunded: i64,
    pub application: ReplaceMeWithAnyOfSpec,
    pub balance_transaction: ReplaceMeWithAnyOfSpec,
    pub charge: ReplaceMeWithAnyOfSpec,
    pub created: i64,
    pub currency: String,
    pub id: String,
    pub livemode: bool,
    pub object: String,
    pub originating_transaction: ReplaceMeWithAnyOfSpec,
    pub refunded: bool,
    pub refunds: ApplicationFeeRefunds,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MandateSingleUse {
    pub amount: i64,
    pub currency: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ExchangeRate {
    pub id: String,
    pub object: String,
    pub rates: ExchangeRateRates,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AccountSession {
    pub account: String,
    pub client_secret: String,
    pub components: ConnectEmbeddedAccountSessionCreateComponents,
    pub expires_at: i64,
    pub livemode: bool,
    pub object: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesBacsDebitPayments {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InvoiceSettingCustomerSetting {
    pub custom_fields: Vec<InvoiceSettingCustomField>,
    pub default_payment_method: ReplaceMeWithAnyOfSpec,
    pub footer: String,
    pub rendering_options: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeletedTerminalConfiguration {
    pub deleted: bool,
    pub id: String,
    pub object: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SourceTypeCardPresent {
    pub application_cryptogram: Option<String>,
    pub application_preferred_name: Option<String>,
    pub authorization_code: Option<String>,
    pub authorization_response_code: Option<String>,
    pub brand: Option<String>,
    pub country: Option<String>,
    pub cvm_type: Option<String>,
    pub data_type: Option<String>,
    pub dedicated_file_name: Option<String>,
    pub description: Option<String>,
    pub emv_auth_data: Option<String>,
    pub evidence_customer_signature: Option<String>,
    pub evidence_transaction_certificate: Option<String>,
    pub exp_month: Option<i64>,
    pub exp_year: Option<i64>,
    pub fingerprint: Option<String>,
    pub funding: Option<String>,
    pub iin: Option<String>,
    pub issuer: Option<String>,
    pub last4: Option<String>,
    pub pos_device_id: Option<String>,
    pub pos_entry_mode: Option<String>,
    pub read_method: Option<String>,
    pub reader: Option<String>,
    pub terminal_verification_results: Option<String>,
    pub transaction_status_information: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    Day,
    Month,
    Sporadic,
    Week,
    Year,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryOutboundTransfer {
    pub amount: i64,
    pub cancelable: bool,
    pub created: i64,
    pub currency: String,
    pub description: String,
    pub destination_payment_method: String,
    pub destination_payment_method_details: OutboundTransfersPaymentMethodDetails,
    pub expected_arrival_date: i64,
    pub financial_account: String,
    pub hosted_regulatory_receipt_url: String,
    pub id: String,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub object: String,
    pub returned_details: ReplaceMeWithAnyOfSpec,
    pub statement_descriptor: String,
    pub status: TreasuryOutboundTransferStatus,
    pub status_transitions: TreasuryOutboundTransfersResourceStatusTransitions,
    pub transaction: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AccountPayoutSettings {
    pub debit_negative_balances: bool,
    pub schedule: TransferSchedule,
    pub statement_descriptor: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects {
    Always,
    Never,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsCardWalletApplePay {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentPaymentMethodOptionsBlik {
    pub setup_future_usage: Option<
        PaymentIntentPaymentMethodOptionsBlikSetupFutureUsage,
    >,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesIncrementalAuthorizationIncrementalAuthorization {
    pub status: PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesIncrementalAuthorizationIncrementalAuthorizationStatus,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BillingPortalSessionLocale {
    Auto,
    Bg,
    Cs,
    Da,
    De,
    El,
    En,
    EnAU,
    EnCA,
    EnGB,
    EnIE,
    EnIN,
    EnNZ,
    EnSG,
    Es,
    Es419,
    Et,
    Fi,
    Fil,
    Fr,
    FrCA,
    Hr,
    Hu,
    Id,
    It,
    Ja,
    Ko,
    Lt,
    Lv,
    Ms,
    Mt,
    Nb,
    Nl,
    Pl,
    Pt,
    PtBR,
    Ro,
    Ru,
    Sk,
    Sl,
    Sv,
    Th,
    Tr,
    Vi,
    Zh,
    ZhHK,
    ZhTW,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingCardholderIndividual {
    pub card_issuing: Option<ReplaceMeWithAnyOfSpec>,
    pub dob: ReplaceMeWithAnyOfSpec,
    pub first_name: String,
    pub last_name: String,
    pub verification: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardholderSpendingLimitInterval {
    AllTime,
    Daily,
    Monthly,
    PerAuthorization,
    Weekly,
    Yearly,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanBillingScheme {
    PerUnit,
    Tiered,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsBlik {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FundingInstructionsBankTransferType {
    EuBankTransfer,
    JpBankTransfer,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuotesResourceAutomaticTaxStatus {
    Complete,
    Failed,
    RequiresLocationInputs,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectAccountReferenceType {
    Account,
    Self_,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentLinksResourceTaxIdCollection {
    pub enabled: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CheckoutCashappPaymentMethodOptions {
    pub setup_future_usage: Option<CheckoutCashappPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingTokenWalletProvider {
    ApplePay,
    GooglePay,
    SamsungPay,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GelatoSelfieReportStatus {
    Unverified,
    Verified,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PersonAdditionalTosAcceptances {
    pub account: PersonAdditionalTosAcceptance,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AccountRequirements {
    pub alternatives: Vec<AccountRequirementsAlternative>,
    pub current_deadline: i64,
    pub currently_due: Vec<String>,
    pub disabled_reason: String,
    pub errors: Vec<AccountRequirementsError>,
    pub eventually_due: Vec<String>,
    pub past_due: Vec<String>,
    pub pending_verification: Vec<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MandateAcssDebitPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PriceTier {
    pub flat_amount: i64,
    pub flat_amount_decimal: String,
    pub unit_amount: i64,
    pub unit_amount_decimal: String,
    pub up_to: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SourceTransactionGbpCreditTransferData {
    pub fingerprint: Option<String>,
    pub funding_method: Option<String>,
    pub last4: Option<String>,
    pub reference: Option<String>,
    pub sender_account_number: Option<String>,
    pub sender_name: Option<String>,
    pub sender_sort_code: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentNextActionDisplayBankTransferInstructionsType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
    UsBankTransfer,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionSchedulesResourceDefaultSettingsBillingCycleAnchor {
    Automatic,
    PhaseStart,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryOutboundPayment {
    pub amount: i64,
    pub cancelable: bool,
    pub created: i64,
    pub currency: String,
    pub customer: String,
    pub description: String,
    pub destination_payment_method: String,
    pub destination_payment_method_details: ReplaceMeWithAnyOfSpec,
    pub end_user_details: ReplaceMeWithAnyOfSpec,
    pub expected_arrival_date: i64,
    pub financial_account: String,
    pub hosted_regulatory_receipt_url: String,
    pub id: String,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub object: String,
    pub returned_details: ReplaceMeWithAnyOfSpec,
    pub statement_descriptor: String,
    pub status: TreasuryOutboundPaymentStatus,
    pub status_transitions: TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitions,
    pub transaction: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsSepaCreditTransfer {
    pub bank_name: String,
    pub bic: String,
    pub iban: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransaction {
    pub refund: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsCardNetworkToken {
    pub used: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AccountCardPaymentsSettings {
    pub decline_on: Option<AccountDeclineChargeOn>,
    pub statement_descriptor_prefix: String,
    pub statement_descriptor_prefix_kana: String,
    pub statement_descriptor_prefix_kanji: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxProductRegistrationsResourceCountryOptionsDefault {
    pub r#type: TaxProductRegistrationsResourceCountryOptionsDefaultType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortalSubscriptionCancelMode {
    AtPeriodEnd,
    Immediately,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodCardWalletMasterpass {
    pub billing_address: ReplaceMeWithAnyOfSpec,
    pub email: String,
    pub name: String,
    pub shipping_address: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingCardholderRequirements {
    pub disabled_reason: IssuingCardholderRequirementsDisabledReason,
    pub past_due: Vec<IssuingCardholderRequirementsPastDueItem>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentNextAction {
    pub alipay_handle_redirect: Option<PaymentIntentNextActionAlipayHandleRedirect>,
    pub boleto_display_details: Option<PaymentIntentNextActionBoleto>,
    pub card_await_notification: Option<PaymentIntentNextActionCardAwaitNotification>,
    pub cashapp_handle_redirect_or_display_qr_code: Option<
        PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCode,
    >,
    pub display_bank_transfer_instructions: Option<
        PaymentIntentNextActionDisplayBankTransferInstructions,
    >,
    pub konbini_display_details: Option<PaymentIntentNextActionKonbini>,
    pub oxxo_display_details: Option<PaymentIntentNextActionDisplayOxxoDetails>,
    pub paynow_display_qr_code: Option<PaymentIntentNextActionPaynowDisplayQrCode>,
    pub pix_display_qr_code: Option<PaymentIntentNextActionPixDisplayQrCode>,
    pub promptpay_display_qr_code: Option<PaymentIntentNextActionPromptpayDisplayQrCode>,
    pub redirect_to_url: Option<PaymentIntentNextActionRedirectToUrl>,
    pub swish_handle_redirect_or_display_qr_code: Option<
        PaymentIntentNextActionSwishHandleRedirectOrDisplayQrCode,
    >,
    pub r#type: String,
    pub use_stripe_sdk: Option<PaymentIntentNextActionUseStripeSdk>,
    pub verify_with_microdeposits: Option<
        PaymentIntentNextActionVerifyWithMicrodeposits,
    >,
    pub wechat_pay_display_qr_code: Option<
        PaymentIntentNextActionWechatPayDisplayQrCode,
    >,
    pub wechat_pay_redirect_to_android_app: Option<
        PaymentIntentNextActionWechatPayRedirectToAndroidApp,
    >,
    pub wechat_pay_redirect_to_ios_app: Option<
        PaymentIntentNextActionWechatPayRedirectToIosApp,
    >,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefundDestinationDetailsCardType {
    Pending,
    Refund,
    Reversal,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodP24 {
    pub bank: PaymentMethodP24Bank,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetupIntentPaymentMethodOptionsPaypal {
    pub billing_agreement_id: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsUsBankAccount {
    pub account_holder_type: PaymentMethodDetailsUsBankAccountAccountHolderType,
    pub account_type: PaymentMethodDetailsUsBankAccountAccountType,
    pub bank_name: String,
    pub fingerprint: String,
    pub last4: String,
    pub routing_number: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SourceOrder {
    pub amount: i64,
    pub currency: String,
    pub email: Option<String>,
    pub items: Vec<SourceOrderItem>,
    pub shipping: Option<Shipping>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    Amounts,
    DescriptorCode,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentNextActionAlipayHandleRedirect {
    pub native_data: String,
    pub native_url: String,
    pub return_url: String,
    pub url: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodOptionsGiropay {
    pub setup_future_usage: Option<PaymentMethodOptionsGiropaySetupFutureUsage>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsBacsDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentPaymentMethodOptionsLink {
    pub capture_method: Option<PaymentIntentPaymentMethodOptionsLinkCaptureMethod>,
    pub persistent_token: String,
    pub setup_future_usage: Option<
        PaymentIntentPaymentMethodOptionsLinkSetupFutureUsage,
    >,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalReaderReaderResourceCart {
    pub currency: String,
    pub line_items: Vec<TerminalReaderReaderResourceLineItem>,
    pub tax: i64,
    pub total: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct QuotesResourceUpfrontLineItems {
    pub data: Vec<Item>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentLinksResourceCustomFieldsDropdownOption {
    pub label: String,
    pub value: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsUsBankAccountAccountHolderType {
    Company,
    Individual,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesCardPayments {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FundingInstructionsBankTransferSortCodeRecord {
    pub account_holder_name: String,
    pub account_number: String,
    pub sort_code: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InvoiceSettingCustomField {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MandateBacsDebitNetworkStatus {
    Accepted,
    Pending,
    Refused,
    Revoked,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingTransaction {
    pub amount: i64,
    pub amount_details: ReplaceMeWithAnyOfSpec,
    pub authorization: ReplaceMeWithAnyOfSpec,
    pub balance_transaction: ReplaceMeWithAnyOfSpec,
    pub card: ReplaceMeWithAnyOfSpec,
    pub cardholder: ReplaceMeWithAnyOfSpec,
    pub created: i64,
    pub currency: String,
    pub dispute: ReplaceMeWithAnyOfSpec,
    pub id: String,
    pub livemode: bool,
    pub merchant_amount: i64,
    pub merchant_currency: String,
    pub merchant_data: IssuingAuthorizationMerchantData,
    pub metadata: HashMap<String, String>,
    pub network_data: ReplaceMeWithAnyOfSpec,
    pub object: String,
    pub purchase_details: ReplaceMeWithAnyOfSpec,
    pub token: Option<ReplaceMeWithAnyOfSpec>,
    pub treasury: Option<ReplaceMeWithAnyOfSpec>,
    pub r#type: IssuingTransactionType,
    pub wallet: IssuingTransactionWallet,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoicesResourceInvoiceTaxIdType {
    AdNrt,
    AeTrn,
    ArCuit,
    AuAbn,
    AuArn,
    BgUic,
    BoTin,
    BrCnpj,
    BrCpf,
    CaBn,
    CaGstHst,
    CaPstBc,
    CaPstMb,
    CaPstSk,
    CaQst,
    ChVat,
    ClTin,
    CnTin,
    CoNit,
    CrTin,
    DoRcn,
    EcRuc,
    EgTin,
    EsCif,
    EuOssVat,
    EuVat,
    GbVat,
    GeVat,
    HkBr,
    HuTin,
    IdNpwp,
    IlVat,
    InGst,
    IsVat,
    JpCn,
    JpRn,
    JpTrn,
    KePin,
    KrBrn,
    LiUid,
    MxRfc,
    MyFrp,
    MyItn,
    MySst,
    NoVat,
    NzGst,
    PeRuc,
    PhTin,
    RoTin,
    RsPib,
    RuInn,
    RuKpp,
    SaVat,
    SgGst,
    SgUen,
    SiTin,
    SvNit,
    ThVat,
    TrTin,
    TwVat,
    UaVat,
    Unknown,
    UsEin,
    UyRuc,
    VeRif,
    VnTin,
    ZaVat,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodOptionsPix {
    pub expires_after_seconds: i64,
    pub expires_at: i64,
    pub setup_future_usage: Option<PaymentMethodOptionsPixSetupFutureUsage>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Card {
    pub account: Option<ReplaceMeWithAnyOfSpec>,
    pub address_city: String,
    pub address_country: String,
    pub address_line1: String,
    pub address_line1_check: String,
    pub address_line2: String,
    pub address_state: String,
    pub address_zip: String,
    pub address_zip_check: String,
    pub available_payout_methods: Option<Vec<CardAvailablePayoutMethodsItem>>,
    pub brand: String,
    pub country: String,
    pub currency: Option<String>,
    pub customer: Option<ReplaceMeWithAnyOfSpec>,
    pub cvc_check: String,
    pub default_for_currency: Option<bool>,
    pub description: Option<String>,
    pub dynamic_last4: String,
    pub exp_month: i64,
    pub exp_year: i64,
    pub fingerprint: Option<String>,
    pub funding: String,
    pub id: String,
    pub iin: Option<String>,
    pub issuer: Option<String>,
    pub last4: String,
    pub metadata: HashMap<String, String>,
    pub name: String,
    pub object: String,
    pub status: Option<String>,
    pub tokenization_method: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntentStatus {
    Canceled,
    Processing,
    RequiresAction,
    RequiresConfirmation,
    RequiresPaymentMethod,
    Succeeded,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeletedPerson {
    pub deleted: bool,
    pub id: String,
    pub object: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutBoletoPaymentMethodOptionsSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FinancialConnectionsAccount {
    pub account_holder: ReplaceMeWithAnyOfSpec,
    pub balance: ReplaceMeWithAnyOfSpec,
    pub balance_refresh: ReplaceMeWithAnyOfSpec,
    pub category: FinancialConnectionsAccountCategory,
    pub created: i64,
    pub display_name: String,
    pub id: String,
    pub institution_name: String,
    pub last4: String,
    pub livemode: bool,
    pub object: String,
    pub ownership: ReplaceMeWithAnyOfSpec,
    pub ownership_refresh: ReplaceMeWithAnyOfSpec,
    pub permissions: Vec<FinancialConnectionsAccountPermissionsItem>,
    pub status: FinancialConnectionsAccountStatus,
    pub subcategory: FinancialConnectionsAccountSubcategory,
    pub subscriptions: Vec<FinancialConnectionsAccountSubscriptionsItem>,
    pub supported_payment_method_types: Vec<
        FinancialConnectionsAccountSupportedPaymentMethodTypesItem,
    >,
    pub transaction_refresh: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingTransactionNetworkData {
    pub authorization_code: String,
    pub processing_date: String,
    pub transaction_id: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesOxxoPayments {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionPendingInvoiceItemIntervalInterval {
    Day,
    Month,
    Week,
    Year,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductRegistrationsResourceCountryOptionsSimplifiedType {
    Simplified,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryReceivedCreditStatus {
    Failed,
    Succeeded,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceRenderingPdfPageSize {
    A4,
    Auto,
    Letter,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
    Challenge,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortalSubscriptionUpdateDefaultAllowedUpdatesItem {
    Price,
    PromotionCode,
    Quantity,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SourceTransactionPaperCheckData {
    pub available_at: Option<String>,
    pub invoices: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetupAttemptPaymentMethodDetailsSofort {
    pub bank_code: String,
    pub bank_name: String,
    pub bic: String,
    pub generated_sepa_debit: ReplaceMeWithAnyOfSpec,
    pub generated_sepa_debit_mandate: ReplaceMeWithAnyOfSpec,
    pub iban_last4: String,
    pub preferred_language: SetupAttemptPaymentMethodDetailsSofortPreferredLanguage,
    pub verified_name: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaxTransactionLineItemType {
    Reversal,
    Transaction,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsCardWalletMasterpass {
    pub billing_address: ReplaceMeWithAnyOfSpec,
    pub email: String,
    pub name: String,
    pub shipping_address: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsInteracPresentReceipt {
    pub account_type: Option<PaymentMethodDetailsInteracPresentReceiptAccountType>,
    pub application_cryptogram: String,
    pub application_preferred_name: String,
    pub authorization_code: String,
    pub authorization_response_code: String,
    pub cardholder_verification_method: String,
    pub dedicated_file_name: String,
    pub terminal_verification_results: String,
    pub transaction_status_information: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalConfigurationConfigurationResourceOfflineConfig {
    pub enabled: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionScheduleStatus {
    Active,
    Canceled,
    Completed,
    NotStarted,
    Released,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CheckoutAffirmPaymentMethodOptions {
    pub setup_future_usage: Option<CheckoutAffirmPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceBillingReason {
    AutomaticPendingInvoiceItemInvoice,
    Manual,
    QuoteAccept,
    Subscription,
    SubscriptionCreate,
    SubscriptionCycle,
    SubscriptionThreshold,
    SubscriptionUpdate,
    Upcoming,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CheckoutCardInstallmentsOptions {
    pub enabled: Option<bool>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InvoiceTaxAmount {
    pub amount: i64,
    pub inclusive: bool,
    pub tax_rate: ReplaceMeWithAnyOfSpec,
    pub taxability_reason: InvoiceTaxAmountTaxabilityReason,
    pub taxable_amount: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodOptionsPromptpay {
    pub setup_future_usage: Option<PaymentMethodOptionsPromptpaySetupFutureUsage>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SubscriptionsTrialsResourceTrialSettings {
    pub end_behavior: SubscriptionsTrialsResourceEndBehavior,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentLinksResourceCustomFieldsText {
    pub maximum_length: i64,
    pub minimum_length: i64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryTransactionFlowType {
    CreditReversal,
    DebitReversal,
    InboundTransfer,
    IssuingAuthorization,
    Other,
    OutboundPayment,
    OutboundTransfer,
    ReceivedCredit,
    ReceivedDebit,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CheckoutAlipayPaymentMethodOptions {
    pub setup_future_usage: Option<CheckoutAlipayPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardCancellationReason {
    DesignRejected,
    Lost,
    Stolen,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CouponCurrencyOptions {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesCardIssuing {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransfer {
    pub account_number_last4: String,
    pub sender_name: String,
    pub sort_code: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FinancialConnectionsSessionAccounts {
    pub data: Vec<FinancialConnectionsAccount>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GelatoVerificationSessionOptions {
    pub document: Option<GelatoSessionDocumentOptions>,
    pub id_number: Option<GelatoSessionIdNumberOptions>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodGiropay {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutPaynowPaymentMethodOptionsSetupFutureUsage {
    None,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionsResourcePauseCollectionBehavior {
    KeepAsDraft,
    MarkUncollectible,
    Void,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingCardholderAuthorizationControls {
    pub allowed_categories: Vec<
        IssuingCardholderAuthorizationControlsAllowedCategoriesItem,
    >,
    pub blocked_categories: Vec<
        IssuingCardholderAuthorizationControlsBlockedCategoriesItem,
    >,
    pub spending_limits: Vec<IssuingCardholderSpendingLimit>,
    pub spending_limits_currency: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CheckoutBancontactPaymentMethodOptions {
    pub setup_future_usage: Option<
        CheckoutBancontactPaymentMethodOptionsSetupFutureUsage,
    >,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsSepaDebit {
    pub bank_code: String,
    pub branch_code: String,
    pub country: String,
    pub fingerprint: String,
    pub last4: String,
    pub mandate: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingTransactionAmountDetails {
    pub atm_fee: i64,
    pub cashback_amount: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionTaxId {
    pub r#type: PaymentPagesCheckoutSessionTaxIdType,
    pub value: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentLinksResourceSubscriptionDataInvoiceSettings {
    pub issuer: ConnectAccountReference,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentPagesCheckoutSessionConsentCollectionTermsOfService {
    None,
    Required,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct QuotesResourceFromQuote {
    pub is_revision: bool,
    pub quote: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingCardAuthorizationControls {
    pub allowed_categories: Vec<IssuingCardAuthorizationControlsAllowedCategoriesItem>,
    pub blocked_categories: Vec<IssuingCardAuthorizationControlsBlockedCategoriesItem>,
    pub spending_limits: Vec<IssuingCardSpendingLimit>,
    pub spending_limits_currency: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct QuotesResourceTransferData {
    pub amount: i64,
    pub amount_percent: f64,
    pub destination: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentFlowsAmountDetails {
    pub tip: Option<PaymentFlowsAmountDetailsResourceTip>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetupAttemptPaymentMethodDetailsCashapp {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LegalEntityUboDeclaration {
    pub date: i64,
    pub ip: String,
    pub user_agent: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ApiErrors {
    pub charge: Option<String>,
    pub code: Option<ApiErrorsCode>,
    pub decline_code: Option<String>,
    pub doc_url: Option<String>,
    pub message: Option<String>,
    pub param: Option<String>,
    pub payment_intent: Option<PaymentIntent>,
    pub payment_method: Option<PaymentMethod>,
    pub payment_method_type: Option<String>,
    pub request_log_url: Option<String>,
    pub setup_intent: Option<SetupIntent>,
    pub source: Option<PaymentSource>,
    pub r#type: ApiErrorsType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalConfigurationConfigurationResourceTipping {
    pub aud: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    pub cad: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    pub chf: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    pub czk: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    pub dkk: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    pub eur: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    pub gbp: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    pub hkd: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    pub myr: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    pub nok: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    pub nzd: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    pub sek: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    pub sgd: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    pub usd: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutAcssDebitMandateOptionsDefaultForItem {
    Invoice,
    Subscription,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ReceivedPaymentMethodDetailsFinancialAccount {
    pub id: String,
    pub network: ReceivedPaymentMethodDetailsFinancialAccountNetwork,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InvoiceSettingRenderingOptions {
    pub amount_tax_display: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsBancontactSetupFutureUsage {
    None,
    OffSession,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentFlowsAutomaticPaymentMethodsSetupIntentAllowRedirects {
    Always,
    Never,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodAffirm {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InvoicesPaymentMethodOptions {
    pub acss_debit: ReplaceMeWithAnyOfSpec,
    pub bancontact: ReplaceMeWithAnyOfSpec,
    pub card: ReplaceMeWithAnyOfSpec,
    pub customer_balance: ReplaceMeWithAnyOfSpec,
    pub konbini: ReplaceMeWithAnyOfSpec,
    pub us_bank_account: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InvoicesPaymentSettings {
    pub default_mandate: String,
    pub payment_method_options: ReplaceMeWithAnyOfSpec,
    pub payment_method_types: Vec<InvoicesPaymentSettingsPaymentMethodTypesItem>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CancellationDetailsFeedback {
    CustomerService,
    LowQuality,
    MissingFeatures,
    Other,
    SwitchedService,
    TooComplex,
    TooExpensive,
    Unused,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentNextActionPixDisplayQrCode {
    pub data: Option<String>,
    pub expires_at: Option<i64>,
    pub hosted_instructions_url: Option<String>,
    pub image_url_png: Option<String>,
    pub image_url_svg: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentNextActionRedirectToUrl {
    pub return_url: String,
    pub url: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsCardPresentOffline {
    pub stored_at: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NotificationEventDataObject {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodConfigResourceDisplayPreference {
    pub overridable: bool,
    pub preference: PaymentMethodConfigResourceDisplayPreferencePreference,
    pub value: PaymentMethodConfigResourceDisplayPreferenceValue,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CustomerTaxIds {
    pub data: Vec<TaxId>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Review {
    pub billing_zip: String,
    pub charge: ReplaceMeWithAnyOfSpec,
    pub closed_reason: ReviewClosedReason,
    pub created: i64,
    pub id: String,
    pub ip_address: String,
    pub ip_address_location: ReplaceMeWithAnyOfSpec,
    pub livemode: bool,
    pub object: String,
    pub open: bool,
    pub opened_reason: ReviewOpenedReason,
    pub payment_intent: Option<ReplaceMeWithAnyOfSpec>,
    pub reason: String,
    pub session: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionShippingCost {
    pub amount_subtotal: i64,
    pub amount_tax: i64,
    pub amount_total: i64,
    pub shipping_rate: ReplaceMeWithAnyOfSpec,
    pub taxes: Option<Vec<LineItemsTaxAmount>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsCustomerBalance {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FundingInstructions {
    pub bank_transfer: FundingInstructionsBankTransfer,
    pub currency: String,
    pub funding_type: FundingInstructionsFundingType,
    pub livemode: bool,
    pub object: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentNextActionKonbiniSeicomart {
    pub confirmation_number: Option<String>,
    pub payment_code: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaxIDsOwnerType {
    Account,
    Application,
    Customer,
    Self_,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryInboundTransfersResourceInboundTransferResourceStatusTransitions {
    pub canceled_at: Option<i64>,
    pub failed_at: i64,
    pub succeeded_at: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingCardWallets {
    pub apple_pay: IssuingCardApplePay,
    pub google_pay: IssuingCardGooglePay,
    pub primary_account_identifier: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkedAccountOptionsUsBankAccountPrefetchItem {
    Balances,
    Transactions,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CashBalanceAvailable {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsGiropay {
    pub bank_code: String,
    pub bank_name: String,
    pub bic: String,
    pub verified_name: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingNetworkTokenWalletProvider {
    pub account_id: Option<String>,
    pub account_trust_score: Option<i64>,
    pub card_number_source: Option<IssuingNetworkTokenWalletProviderCardNumberSource>,
    pub cardholder_address: Option<IssuingNetworkTokenAddress>,
    pub cardholder_name: Option<String>,
    pub device_trust_score: Option<i64>,
    pub hashed_account_email_address: Option<String>,
    pub reason_codes: Option<Vec<IssuingNetworkTokenWalletProviderReasonCodesItem>>,
    pub suggested_decision: Option<IssuingNetworkTokenWalletProviderSuggestedDecision>,
    pub suggested_decision_version: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxCalculationLineItems {
    pub data: Vec<TaxCalculationLineItem>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxCode {
    pub description: String,
    pub id: String,
    pub name: String,
    pub object: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GelatoSessionIdNumberOptions {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingAuthorizationVerificationData {
    pub address_line1_check: IssuingAuthorizationVerificationDataAddressLine1Check,
    pub address_postal_code_check: IssuingAuthorizationVerificationDataAddressPostalCodeCheck,
    pub authentication_exemption: ReplaceMeWithAnyOfSpec,
    pub cvc_check: IssuingAuthorizationVerificationDataCvcCheck,
    pub expiry_check: IssuingAuthorizationVerificationDataExpiryCheck,
    pub postal_code: String,
    pub three_d_secure: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InvoiceSettingSubscriptionSchedulePhaseSetting {
    pub account_tax_ids: Option<Vec<ReplaceMeWithAnyOfSpec>>,
    pub days_until_due: i64,
    pub issuer: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TransformUsage {
    pub divide_by: i64,
    pub round: TransformUsageRound,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FundingInstructionsBankTransfer {
    pub country: String,
    pub financial_addresses: Vec<FundingInstructionsBankTransferFinancialAddress>,
    pub r#type: FundingInstructionsBankTransferType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodAuBecsDebit {
    pub bsb_number: String,
    pub fingerprint: String,
    pub last4: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardAuthorizationControlsBlockedCategoriesItem {
    AcRefrigerationRepair,
    AccountingBookkeepingServices,
    AdvertisingServices,
    AgriculturalCooperative,
    AirlinesAirCarriers,
    AirportsFlyingFields,
    AmbulanceServices,
    AmusementParksCarnivals,
    AntiqueReproductions,
    AntiqueShops,
    Aquariums,
    ArchitecturalSurveyingServices,
    ArtDealersAndGalleries,
    ArtistsSupplyAndCraftShops,
    AutoAndHomeSupplyStores,
    AutoBodyRepairShops,
    AutoPaintShops,
    AutoServiceShops,
    AutomatedCashDisburse,
    AutomatedFuelDispensers,
    AutomobileAssociations,
    AutomotivePartsAndAccessoriesStores,
    AutomotiveTireStores,
    BailAndBondPayments,
    Bakeries,
    BandsOrchestras,
    BarberAndBeautyShops,
    BettingCasinoGambling,
    BicycleShops,
    BilliardPoolEstablishments,
    BoatDealers,
    BoatRentalsAndLeases,
    BookStores,
    BooksPeriodicalsAndNewspapers,
    BowlingAlleys,
    BusLines,
    BusinessSecretarialSchools,
    BuyingShoppingServices,
    CableSatelliteAndOtherPayTelevisionAndRadio,
    CameraAndPhotographicSupplyStores,
    CandyNutAndConfectioneryStores,
    CarAndTruckDealersNewUsed,
    CarAndTruckDealersUsedOnly,
    CarRentalAgencies,
    CarWashes,
    CarpentryServices,
    CarpetUpholsteryCleaning,
    Caterers,
    CharitableAndSocialServiceOrganizationsFundraising,
    ChemicalsAndAlliedProducts,
    ChildCareServices,
    ChildrensAndInfantsWearStores,
    ChiropodistsPodiatrists,
    Chiropractors,
    CigarStoresAndStands,
    CivicSocialFraternalAssociations,
    CleaningAndMaintenance,
    ClothingRental,
    CollegesUniversities,
    CommercialEquipment,
    CommercialFootwear,
    CommercialPhotographyArtAndGraphics,
    CommuterTransportAndFerries,
    ComputerNetworkServices,
    ComputerProgramming,
    ComputerRepair,
    ComputerSoftwareStores,
    ComputersPeripheralsAndSoftware,
    ConcreteWorkServices,
    ConstructionMaterials,
    ConsultingPublicRelations,
    CorrespondenceSchools,
    CosmeticStores,
    CounselingServices,
    CountryClubs,
    CourierServices,
    CourtCosts,
    CreditReportingAgencies,
    CruiseLines,
    DairyProductsStores,
    DanceHallStudiosSchools,
    DatingEscortServices,
    DentistsOrthodontists,
    DepartmentStores,
    DetectiveAgencies,
    DigitalGoodsApplications,
    DigitalGoodsGames,
    DigitalGoodsLargeVolume,
    DigitalGoodsMedia,
    DirectMarketingCatalogMerchant,
    DirectMarketingCombinationCatalogAndRetailMerchant,
    DirectMarketingInboundTelemarketing,
    DirectMarketingInsuranceServices,
    DirectMarketingOther,
    DirectMarketingOutboundTelemarketing,
    DirectMarketingSubscription,
    DirectMarketingTravel,
    DiscountStores,
    Doctors,
    DoorToDoorSales,
    DraperyWindowCoveringAndUpholsteryStores,
    DrinkingPlaces,
    DrugStoresAndPharmacies,
    DrugsDrugProprietariesAndDruggistSundries,
    DryCleaners,
    DurableGoods,
    DutyFreeStores,
    EatingPlacesRestaurants,
    EducationalServices,
    ElectricRazorStores,
    ElectricVehicleCharging,
    ElectricalPartsAndEquipment,
    ElectricalServices,
    ElectronicsRepairShops,
    ElectronicsStores,
    ElementarySecondarySchools,
    EmergencyServicesGcasVisaUseOnly,
    EmploymentTempAgencies,
    EquipmentRental,
    ExterminatingServices,
    FamilyClothingStores,
    FastFoodRestaurants,
    FinancialInstitutions,
    FinesGovernmentAdministrativeEntities,
    FireplaceFireplaceScreensAndAccessoriesStores,
    FloorCoveringStores,
    Florists,
    FloristsSuppliesNurseryStockAndFlowers,
    FreezerAndLockerMeatProvisioners,
    FuelDealersNonAutomotive,
    FuneralServicesCrematories,
    FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances,
    FurnitureRepairRefinishing,
    FurriersAndFurShops,
    GeneralServices,
    GiftCardNoveltyAndSouvenirShops,
    GlassPaintAndWallpaperStores,
    GlasswareCrystalStores,
    GolfCoursesPublic,
    GovernmentLicensedHorseDogRacingUsRegionOnly,
    GovernmentLicensedOnlineCasionsOnlineGamblingUsRegionOnly,
    GovernmentOwnedLotteriesNonUsRegion,
    GovernmentOwnedLotteriesUsRegionOnly,
    GovernmentServices,
    GroceryStoresSupermarkets,
    HardwareEquipmentAndSupplies,
    HardwareStores,
    HealthAndBeautySpas,
    HearingAidsSalesAndSupplies,
    HeatingPlumbingAC,
    HobbyToyAndGameShops,
    HomeSupplyWarehouseStores,
    Hospitals,
    HotelsMotelsAndResorts,
    HouseholdApplianceStores,
    IndustrialSupplies,
    InformationRetrievalServices,
    InsuranceDefault,
    InsuranceUnderwritingPremiums,
    IntraCompanyPurchases,
    JewelryStoresWatchesClocksAndSilverwareStores,
    LandscapingServices,
    Laundries,
    LaundryCleaningServices,
    LegalServicesAttorneys,
    LuggageAndLeatherGoodsStores,
    LumberBuildingMaterialsStores,
    ManualCashDisburse,
    MarinasServiceAndSupplies,
    Marketplaces,
    MasonryStoneworkAndPlaster,
    MassageParlors,
    MedicalAndDentalLabs,
    MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies,
    MedicalServices,
    MembershipOrganizations,
    MensAndBoysClothingAndAccessoriesStores,
    MensWomensClothingStores,
    MetalServiceCenters,
    Miscellaneous,
    MiscellaneousApparelAndAccessoryShops,
    MiscellaneousAutoDealers,
    MiscellaneousBusinessServices,
    MiscellaneousFoodStores,
    MiscellaneousGeneralMerchandise,
    MiscellaneousGeneralServices,
    MiscellaneousHomeFurnishingSpecialtyStores,
    MiscellaneousPublishingAndPrinting,
    MiscellaneousRecreationServices,
    MiscellaneousRepairShops,
    MiscellaneousSpecialtyRetail,
    MobileHomeDealers,
    MotionPictureTheaters,
    MotorFreightCarriersAndTrucking,
    MotorHomesDealers,
    MotorVehicleSuppliesAndNewParts,
    MotorcycleShopsAndDealers,
    MotorcycleShopsDealers,
    MusicStoresMusicalInstrumentsPianosAndSheetMusic,
    NewsDealersAndNewsstands,
    NonFiMoneyOrders,
    NonFiStoredValueCardPurchaseLoad,
    NondurableGoods,
    NurseriesLawnAndGardenSupplyStores,
    NursingPersonalCare,
    OfficeAndCommercialFurniture,
    OpticiansEyeglasses,
    OptometristsOphthalmologist,
    OrthopedicGoodsProstheticDevices,
    Osteopaths,
    PackageStoresBeerWineAndLiquor,
    PaintsVarnishesAndSupplies,
    ParkingLotsGarages,
    PassengerRailways,
    PawnShops,
    PetShopsPetFoodAndSupplies,
    PetroleumAndPetroleumProducts,
    PhotoDeveloping,
    PhotographicPhotocopyMicrofilmEquipmentAndSupplies,
    PhotographicStudios,
    PictureVideoProduction,
    PieceGoodsNotionsAndOtherDryGoods,
    PlumbingHeatingEquipmentAndSupplies,
    PoliticalOrganizations,
    PostalServicesGovernmentOnly,
    PreciousStonesAndMetalsWatchesAndJewelry,
    ProfessionalServices,
    PublicWarehousingAndStorage,
    QuickCopyReproAndBlueprint,
    Railroads,
    RealEstateAgentsAndManagersRentals,
    RecordStores,
    RecreationalVehicleRentals,
    ReligiousGoodsStores,
    ReligiousOrganizations,
    RoofingSidingSheetMetal,
    SecretarialSupportServices,
    SecurityBrokersDealers,
    ServiceStations,
    SewingNeedleworkFabricAndPieceGoodsStores,
    ShoeRepairHatCleaning,
    ShoeStores,
    SmallApplianceRepair,
    SnowmobileDealers,
    SpecialTradeServices,
    SpecialtyCleaning,
    SportingGoodsStores,
    SportingRecreationCamps,
    SportsAndRidingApparelStores,
    SportsClubsFields,
    StampAndCoinStores,
    StationaryOfficeSuppliesPrintingAndWritingPaper,
    StationeryStoresOfficeAndSchoolSupplyStores,
    SwimmingPoolsSales,
    TUiTravelGermany,
    TailorsAlterations,
    TaxPaymentsGovernmentAgencies,
    TaxPreparationServices,
    TaxicabsLimousines,
    TelecommunicationEquipmentAndTelephoneSales,
    TelecommunicationServices,
    TelegraphServices,
    TentAndAwningShops,
    TestingLaboratories,
    TheatricalTicketAgencies,
    Timeshares,
    TireRetreadingAndRepair,
    TollsBridgeFees,
    TouristAttractionsAndExhibits,
    TowingServices,
    TrailerParksCampgrounds,
    TransportationServices,
    TravelAgenciesTourOperators,
    TruckStopIteration,
    TruckUtilityTrailerRentals,
    TypesettingPlateMakingAndRelatedServices,
    TypewriterStores,
    USFederalGovernmentAgenciesOrDepartments,
    UniformsCommercialClothing,
    UsedMerchandiseAndSecondhandStores,
    Utilities,
    VarietyStores,
    VeterinaryServices,
    VideoAmusementGameSupplies,
    VideoGameArcades,
    VideoTapeRentalStores,
    VocationalTradeSchools,
    WatchJewelryRepair,
    WeldingRepair,
    WholesaleClubs,
    WigAndToupeeStores,
    WiresMoneyOrders,
    WomensAccessoryAndSpecialtyShops,
    WomensReadyToWearStores,
    WreckingAndSalvageYards,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsBancontactPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryInboundTransfer {
    pub amount: i64,
    pub cancelable: bool,
    pub created: i64,
    pub currency: String,
    pub description: String,
    pub failure_details: ReplaceMeWithAnyOfSpec,
    pub financial_account: String,
    pub hosted_regulatory_receipt_url: String,
    pub id: String,
    pub linked_flows: TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlows,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub object: String,
    pub origin_payment_method: String,
    pub origin_payment_method_details: ReplaceMeWithAnyOfSpec,
    pub returned: bool,
    pub statement_descriptor: String,
    pub status: TreasuryInboundTransferStatus,
    pub status_transitions: TreasuryInboundTransfersResourceInboundTransferResourceStatusTransitions,
    pub transaction: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryOutboundPaymentsResourceReturnedStatusCode {
    AccountClosed,
    AccountFrozen,
    BankAccountRestricted,
    BankOwnershipChanged,
    Declined,
    IncorrectAccountHolderName,
    InvalidAccountNumber,
    InvalidCurrency,
    NoAccount,
    Other,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetupIntentPaymentMethodOptionsMandateOptionsSepaDebit {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UsBankAccountNetworks {
    pub preferred: String,
    pub supported: Vec<UsBankAccountNetworksSupportedItem>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GelatoSessionLastErrorCode {
    Abandoned,
    ConsentDeclined,
    CountryNotSupported,
    DeviceNotSupported,
    DocumentExpired,
    DocumentTypeNotSupported,
    DocumentUnverifiedOther,
    IdNumberInsufficientDocumentData,
    IdNumberMismatch,
    IdNumberUnverifiedOther,
    SelfieDocumentMissingPhoto,
    SelfieFaceMismatch,
    SelfieManipulated,
    SelfieUnverifiedOther,
    UnderSupportedAge,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsPaypalCaptureMethod {
    Manual,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    Business,
    Personal,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GelatoIdNumberReportIdNumberType {
    BrCpf,
    SgNric,
    UsSsn,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization {
    IfAvailable,
    Never,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InvoicePaymentMethodOptionsKonbini {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProductType {
    Good,
    Service,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LoginLink {
    pub created: i64,
    pub object: String,
    pub url: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingAuthorizationThreeDSecureResult {
    AttemptAcknowledged,
    Authenticated,
    Failed,
    Required,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CustomerTaxExempt {
    Exempt,
    None,
    Reverse,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsGrabpaySetupFutureUsage {
    None,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardShippingType {
    Bulk,
    Individual,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceTransferredToBalance {
    pub balance_transaction: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalReaderReaderResourceReaderActionStatus {
    Failed,
    InProgress,
    Succeeded,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TransferReversal {
    pub amount: i64,
    pub balance_transaction: ReplaceMeWithAnyOfSpec,
    pub created: i64,
    pub currency: String,
    pub destination_payment_refund: ReplaceMeWithAnyOfSpec,
    pub id: String,
    pub metadata: HashMap<String, String>,
    pub object: String,
    pub source_refund: ReplaceMeWithAnyOfSpec,
    pub transfer: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryTransactionEntryType {
    CreditReversal,
    CreditReversalPosting,
    DebitReversal,
    InboundTransfer,
    InboundTransferReturn,
    IssuingAuthorizationHold,
    IssuingAuthorizationRelease,
    Other,
    OutboundPayment,
    OutboundPaymentCancellation,
    OutboundPaymentFailure,
    OutboundPaymentPosting,
    OutboundPaymentReturn,
    OutboundTransfer,
    OutboundTransferCancellation,
    OutboundTransferFailure,
    OutboundTransferPosting,
    OutboundTransferReturn,
    ReceivedCredit,
    ReceivedDebit,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesGiropayPayments {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClimateSupplierRemovalPathway {
    BiomassCarbonRemovalAndStorage,
    DirectAirCapture,
    EnhancedWeathering,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ConnectEmbeddedPayoutsConfig {
    pub enabled: bool,
    pub features: ConnectEmbeddedPayoutsFeatures,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CheckoutEpsPaymentMethodOptions {
    pub setup_future_usage: Option<CheckoutEpsPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsCustomerBalanceBankTransferType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
    UsBankTransfer,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InboundTransfersType {
    UsBankAccount,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentNextActionKonbiniMinistop {
    pub confirmation_number: Option<String>,
    pub payment_code: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceUsBankTransferNetwork {
    Ach,
    DomesticWireUs,
    Swift,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InvoiceLineItemPeriod {
    pub end: i64,
    pub start: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodRevolutPay {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
    UsBankTransfer,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PromotionCodesResourceRestrictionsCurrencyOptions {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransaction {
    pub payment_intent: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BankConnectionsResourceOwnershipRefreshStatus {
    Failed,
    Pending,
    Succeeded,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentLinksResourceCustomFieldsNumeric {
    pub maximum_length: i64,
    pub minimum_length: i64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentPaymentMethodOptionsAuBecsDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsIdeal {
    pub bank: PaymentMethodDetailsIdealBank,
    pub bic: PaymentMethodDetailsIdealBic,
    pub generated_sepa_debit: ReplaceMeWithAnyOfSpec,
    pub generated_sepa_debit_mandate: ReplaceMeWithAnyOfSpec,
    pub iban_last4: String,
    pub verified_name: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SourceTypeAcssDebit {
    pub bank_address_city: Option<String>,
    pub bank_address_line_1: Option<String>,
    pub bank_address_line_2: Option<String>,
    pub bank_address_postal_code: Option<String>,
    pub bank_name: Option<String>,
    pub category: Option<String>,
    pub country: Option<String>,
    pub fingerprint: Option<String>,
    pub last4: Option<String>,
    pub routing_number: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SubscriptionPendingInvoiceItemInterval {
    pub interval: SubscriptionPendingInvoiceItemIntervalInterval,
    pub interval_count: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FinancialConnectionsSession {
    pub account_holder: ReplaceMeWithAnyOfSpec,
    pub accounts: FinancialConnectionsSessionAccounts,
    pub client_secret: String,
    pub filters: Option<BankConnectionsResourceLinkAccountSessionFilters>,
    pub id: String,
    pub livemode: bool,
    pub object: String,
    pub permissions: Vec<FinancialConnectionsSessionPermissionsItem>,
    pub prefetch: Vec<FinancialConnectionsSessionPrefetchItem>,
    pub return_url: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodGrabpay {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryReceivedDebitsResourceReversalDetails {
    pub deadline: i64,
    pub restricted_reason: TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AccountCardIssuingSettings {
    pub tos_acceptance: Option<CardIssuingAccountTermsOfService>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem {
    AC,
    AD,
    AE,
    AF,
    AG,
    AI,
    AL,
    AM,
    AO,
    AQ,
    AR,
    AT,
    AU,
    AW,
    AX,
    AZ,
    BA,
    BB,
    BD,
    BE,
    BF,
    BG,
    BH,
    BI,
    BJ,
    BL,
    BM,
    BN,
    BO,
    BQ,
    BR,
    BS,
    BT,
    BV,
    BW,
    BY,
    BZ,
    CA,
    CD,
    CF,
    CG,
    CH,
    CI,
    CK,
    CL,
    CM,
    CN,
    CO,
    CR,
    CV,
    CW,
    CY,
    CZ,
    DE,
    DJ,
    DK,
    DM,
    DO,
    DZ,
    EC,
    EE,
    EG,
    EH,
    ER,
    ES,
    ET,
    FI,
    FJ,
    FK,
    FO,
    FR,
    GA,
    GB,
    GD,
    GE,
    GF,
    GG,
    GH,
    GI,
    GL,
    GM,
    GN,
    GP,
    GQ,
    GR,
    GS,
    GT,
    GU,
    GW,
    GY,
    HK,
    HN,
    HR,
    HT,
    HU,
    ID,
    IE,
    IL,
    IM,
    IN,
    IO,
    IQ,
    IS,
    IT,
    JE,
    JM,
    JO,
    JP,
    KE,
    KG,
    KH,
    KI,
    KM,
    KN,
    KR,
    KW,
    KY,
    KZ,
    LA,
    LB,
    LC,
    LI,
    LK,
    LR,
    LS,
    LT,
    LU,
    LV,
    LY,
    MA,
    MC,
    MD,
    ME,
    MF,
    MG,
    MK,
    ML,
    MM,
    MN,
    MO,
    MQ,
    MR,
    MS,
    MT,
    MU,
    MV,
    MW,
    MX,
    MY,
    MZ,
    NA,
    NC,
    NE,
    NG,
    NI,
    NL,
    NO,
    NP,
    NR,
    NU,
    NZ,
    OM,
    PA,
    PE,
    PF,
    PG,
    PH,
    PK,
    PL,
    PM,
    PN,
    PR,
    PS,
    PT,
    PY,
    QA,
    RE,
    RO,
    RS,
    RU,
    RW,
    SA,
    SB,
    SC,
    SE,
    SG,
    SH,
    SI,
    SJ,
    SK,
    SL,
    SM,
    SN,
    SO,
    SR,
    SS,
    ST,
    SV,
    SX,
    SZ,
    TA,
    TC,
    TD,
    TF,
    TG,
    TH,
    TJ,
    TK,
    TL,
    TM,
    TN,
    TO,
    TR,
    TT,
    TV,
    TW,
    TZ,
    UA,
    UG,
    US,
    UY,
    UZ,
    VA,
    VC,
    VE,
    VG,
    VN,
    VU,
    WF,
    WS,
    XK,
    YE,
    YT,
    ZA,
    ZM,
    ZW,
    ZZ,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetupIntentTypeSpecificPaymentMethodOptionsClient {
    pub verification_method: Option<
        SetupIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod,
    >,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SubscriptionSchedulesResourceDefaultSettings {
    pub application_fee_percent: f64,
    pub automatic_tax: Option<SubscriptionSchedulesResourceDefaultSettingsAutomaticTax>,
    pub billing_cycle_anchor: SubscriptionSchedulesResourceDefaultSettingsBillingCycleAnchor,
    pub billing_thresholds: ReplaceMeWithAnyOfSpec,
    pub collection_method: SubscriptionSchedulesResourceDefaultSettingsCollectionMethod,
    pub default_payment_method: ReplaceMeWithAnyOfSpec,
    pub description: String,
    pub invoice_settings: InvoiceSettingSubscriptionScheduleSetting,
    pub on_behalf_of: ReplaceMeWithAnyOfSpec,
    pub transfer_data: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FinancialConnectionsAccountOwnershipOwners {
    pub data: Vec<FinancialConnectionsAccountOwner>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodSofort {
    pub country: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SepaDebitGeneratedFrom {
    pub charge: ReplaceMeWithAnyOfSpec,
    pub setup_attempt: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryFinancialAccountsResourceStatusDetails {
    pub closed: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CustomerCashBalanceTransactionType {
    AdjustedForOverdraft,
    AppliedToPayment,
    Funded,
    FundingReversed,
    RefundedFromPayment,
    ReturnCanceled,
    ReturnInitiated,
    TransferredToBalance,
    UnappliedFromPayment,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodSwish {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceUsBankTransfer {
    pub network: Option<
        CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceUsBankTransferNetwork,
    >,
    pub sender_name: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetails {
    pub ip_address: String,
    pub present: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentLinksResourceSubscriptionData {
    pub description: String,
    pub invoice_settings: PaymentLinksResourceSubscriptionDataInvoiceSettings,
    pub metadata: HashMap<String, String>,
    pub trial_period_days: i64,
    pub trial_settings: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MandateLink {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InboundTransfers {
    pub billing_details: TreasurySharedResourceBillingDetails,
    pub r#type: InboundTransfersType,
    pub us_bank_account: Option<InboundTransfersPaymentMethodDetailsUsBankAccount>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxTransactionLineItem {
    pub amount: i64,
    pub amount_tax: i64,
    pub id: String,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub object: String,
    pub product: String,
    pub quantity: i64,
    pub reference: String,
    pub reversal: ReplaceMeWithAnyOfSpec,
    pub tax_behavior: TaxTransactionLineItemTaxBehavior,
    pub tax_code: String,
    pub r#type: TaxTransactionLineItemType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentTypeSpecificPaymentMethodOptionsClientSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodInteracPresentReadMethod {
    ContactEmv,
    ContactlessEmv,
    ContactlessMagstripeMode,
    MagneticStripeFallback,
    MagneticStripeTrack2,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodPaypal {
    pub payer_email: String,
    pub payer_id: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AccountBrandingSettings {
    pub icon: ReplaceMeWithAnyOfSpec,
    pub logo: ReplaceMeWithAnyOfSpec,
    pub primary_color: String,
    pub secondary_color: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CouponAppliesTo {
    pub products: Vec<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeletedRadarValueList {
    pub deleted: bool,
    pub id: String,
    pub object: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PayoutType {
    BankAccount,
    Card,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionScheduleEndBehavior {
    Cancel,
    None,
    Release,
    Renew,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeletedPlan {
    pub deleted: bool,
    pub id: String,
    pub object: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentPaymentMethodOptionsEpsSetupFutureUsage {
    None,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodCardWallet {
    pub amex_express_checkout: Option<PaymentMethodCardWalletAmexExpressCheckout>,
    pub apple_pay: Option<PaymentMethodCardWalletApplePay>,
    pub dynamic_last4: String,
    pub google_pay: Option<PaymentMethodCardWalletGooglePay>,
    pub link: Option<PaymentMethodCardWalletLink>,
    pub masterpass: Option<PaymentMethodCardWalletMasterpass>,
    pub samsung_pay: Option<PaymentMethodCardWalletSamsungPay>,
    pub r#type: PaymentMethodCardWalletType,
    pub visa_checkout: Option<PaymentMethodCardWalletVisaCheckout>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxProductResourceTaxSettingsHeadOffice {
    pub address: Address,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutSessionBillingAddressCollection {
    Auto,
    Required,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Address {
    pub city: String,
    pub country: String,
    pub line1: String,
    pub line2: String,
    pub postal_code: String,
    pub state: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SourceMandateNotificationAcssDebitData {
    pub statement_descriptor: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsInteracPresent {
    pub brand: String,
    pub cardholder_name: String,
    pub country: String,
    pub description: Option<String>,
    pub emv_auth_data: String,
    pub exp_month: i64,
    pub exp_year: i64,
    pub fingerprint: String,
    pub funding: String,
    pub generated_card: String,
    pub iin: Option<String>,
    pub issuer: Option<String>,
    pub last4: String,
    pub network: String,
    pub preferred_locales: Vec<String>,
    pub read_method: PaymentMethodDetailsInteracPresentReadMethod,
    pub receipt: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CountrySpecVerificationFields {
    pub company: CountrySpecVerificationFieldDetails,
    pub individual: CountrySpecVerificationFieldDetails,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CurrencyOption {
    pub custom_unit_amount: ReplaceMeWithAnyOfSpec,
    pub tax_behavior: CurrencyOptionTaxBehavior,
    pub tiers: Option<Vec<PriceTier>>,
    pub unit_amount: i64,
    pub unit_amount_decimal: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GelatoDataDocumentReportDateOfBirth {
    pub day: i64,
    pub month: i64,
    pub year: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetupAttemptPaymentMethodDetailsPaypal {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingDisputeCanceledEvidence {
    pub additional_documentation: ReplaceMeWithAnyOfSpec,
    pub canceled_at: i64,
    pub cancellation_policy_provided: bool,
    pub cancellation_reason: String,
    pub expected_at: i64,
    pub explanation: String,
    pub product_description: String,
    pub product_type: IssuingDisputeCanceledEvidenceProductType,
    pub return_status: IssuingDisputeCanceledEvidenceReturnStatus,
    pub returned_at: i64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InboundTransfersPaymentMethodDetailsUsBankAccountAccountType {
    Checking,
    Savings,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentPaymentMethodOptionsCardRequestIncrementalAuthorization {
    IfAvailable,
    Never,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BankConnectionsResourceTransactionResourceStatusTransitions {
    pub posted_at: i64,
    pub void_at: i64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    AccountApplicationAuthorized,
    AccountApplicationDeauthorized,
    AccountExternalAccountCreated,
    AccountExternalAccountDeleted,
    AccountExternalAccountUpdated,
    AccountUpdated,
    ApplicationFeeCreated,
    ApplicationFeeRefundUpdated,
    ApplicationFeeRefunded,
    BalanceAvailable,
    BillingPortalConfigurationCreated,
    BillingPortalConfigurationUpdated,
    BillingPortalSessionCreated,
    CapabilityUpdated,
    CashBalanceFundsAvailable,
    ChargeCaptured,
    ChargeDisputeClosed,
    ChargeDisputeCreated,
    ChargeDisputeFundsReinstated,
    ChargeDisputeFundsWithdrawn,
    ChargeDisputeUpdated,
    ChargeExpired,
    ChargeFailed,
    ChargePending,
    ChargeRefundUpdated,
    ChargeRefunded,
    ChargeSucceeded,
    ChargeUpdated,
    CheckoutSessionAsyncPaymentFailed,
    CheckoutSessionAsyncPaymentSucceeded,
    CheckoutSessionCompleted,
    CheckoutSessionExpired,
    ClimateOrderCanceled,
    ClimateOrderCreated,
    ClimateOrderDelayed,
    ClimateOrderDelivered,
    ClimateOrderProductSubstituted,
    ClimateProductCreated,
    ClimateProductPricingUpdated,
    CouponCreated,
    CouponDeleted,
    CouponUpdated,
    CreditNoteCreated,
    CreditNoteUpdated,
    CreditNoteVoided,
    CustomerCreated,
    CustomerDeleted,
    CustomerDiscountCreated,
    CustomerDiscountDeleted,
    CustomerDiscountUpdated,
    CustomerSourceCreated,
    CustomerSourceDeleted,
    CustomerSourceExpiring,
    CustomerSourceUpdated,
    CustomerSubscriptionCreated,
    CustomerSubscriptionDeleted,
    CustomerSubscriptionPaused,
    CustomerSubscriptionPendingUpdateApplied,
    CustomerSubscriptionPendingUpdateExpired,
    CustomerSubscriptionResumed,
    CustomerSubscriptionTrialWillEnd,
    CustomerSubscriptionUpdated,
    CustomerTaxIdCreated,
    CustomerTaxIdDeleted,
    CustomerTaxIdUpdated,
    CustomerUpdated,
    CustomerCashBalanceTransactionCreated,
    FileCreated,
    FinancialConnectionsAccountCreated,
    FinancialConnectionsAccountDeactivated,
    FinancialConnectionsAccountDisconnected,
    FinancialConnectionsAccountReactivated,
    FinancialConnectionsAccountRefreshedBalance,
    FinancialConnectionsAccountRefreshedTransactions,
    IdentityVerificationSessionCanceled,
    IdentityVerificationSessionCreated,
    IdentityVerificationSessionProcessing,
    IdentityVerificationSessionRedacted,
    IdentityVerificationSessionRequiresInput,
    IdentityVerificationSessionVerified,
    InvoiceCreated,
    InvoiceDeleted,
    InvoiceFinalizationFailed,
    InvoiceFinalized,
    InvoiceMarkedUncollectible,
    InvoicePaid,
    InvoicePaymentActionRequired,
    InvoicePaymentFailed,
    InvoicePaymentSucceeded,
    InvoiceSent,
    InvoiceUpcoming,
    InvoiceUpdated,
    InvoiceVoided,
    InvoiceitemCreated,
    InvoiceitemDeleted,
    IssuingAuthorizationCreated,
    IssuingAuthorizationRequest,
    IssuingAuthorizationUpdated,
    IssuingCardCreated,
    IssuingCardUpdated,
    IssuingCardholderCreated,
    IssuingCardholderUpdated,
    IssuingDisputeClosed,
    IssuingDisputeCreated,
    IssuingDisputeFundsReinstated,
    IssuingDisputeSubmitted,
    IssuingDisputeUpdated,
    IssuingTokenCreated,
    IssuingTokenUpdated,
    IssuingTransactionCreated,
    IssuingTransactionUpdated,
    MandateUpdated,
    PaymentIntentAmountCapturableUpdated,
    PaymentIntentCanceled,
    PaymentIntentCreated,
    PaymentIntentPartiallyFunded,
    PaymentIntentPaymentFailed,
    PaymentIntentProcessing,
    PaymentIntentRequiresAction,
    PaymentIntentSucceeded,
    PaymentLinkCreated,
    PaymentLinkUpdated,
    PaymentMethodAttached,
    PaymentMethodAutomaticallyUpdated,
    PaymentMethodDetached,
    PaymentMethodUpdated,
    PayoutCanceled,
    PayoutCreated,
    PayoutFailed,
    PayoutPaid,
    PayoutReconciliationCompleted,
    PayoutUpdated,
    PersonCreated,
    PersonDeleted,
    PersonUpdated,
    PlanCreated,
    PlanDeleted,
    PlanUpdated,
    PriceCreated,
    PriceDeleted,
    PriceUpdated,
    ProductCreated,
    ProductDeleted,
    ProductUpdated,
    PromotionCodeCreated,
    PromotionCodeUpdated,
    QuoteAccepted,
    QuoteCanceled,
    QuoteCreated,
    QuoteFinalized,
    RadarEarlyFraudWarningCreated,
    RadarEarlyFraudWarningUpdated,
    RefundCreated,
    RefundUpdated,
    ReportingReportRunFailed,
    ReportingReportRunSucceeded,
    ReportingReportTypeUpdated,
    ReviewClosed,
    ReviewOpened,
    SetupIntentCanceled,
    SetupIntentCreated,
    SetupIntentRequiresAction,
    SetupIntentSetupFailed,
    SetupIntentSucceeded,
    SigmaScheduledQueryRunCreated,
    SourceCanceled,
    SourceChargeable,
    SourceFailed,
    SourceMandateNotification,
    SourceRefundAttributesRequired,
    SourceTransactionCreated,
    SourceTransactionUpdated,
    SubscriptionScheduleAborted,
    SubscriptionScheduleCanceled,
    SubscriptionScheduleCompleted,
    SubscriptionScheduleCreated,
    SubscriptionScheduleExpiring,
    SubscriptionScheduleReleased,
    SubscriptionScheduleUpdated,
    TaxSettingsUpdated,
    TaxRateCreated,
    TaxRateUpdated,
    TerminalReaderActionFailed,
    TerminalReaderActionSucceeded,
    TestHelpersTestClockAdvancing,
    TestHelpersTestClockCreated,
    TestHelpersTestClockDeleted,
    TestHelpersTestClockInternalFailure,
    TestHelpersTestClockReady,
    TopupCanceled,
    TopupCreated,
    TopupFailed,
    TopupReversed,
    TopupSucceeded,
    TransferCreated,
    TransferReversed,
    TransferUpdated,
    TreasuryCreditReversalCreated,
    TreasuryCreditReversalPosted,
    TreasuryDebitReversalCompleted,
    TreasuryDebitReversalCreated,
    TreasuryDebitReversalInitialCreditGranted,
    TreasuryFinancialAccountClosed,
    TreasuryFinancialAccountCreated,
    TreasuryFinancialAccountFeaturesStatusUpdated,
    TreasuryInboundTransferCanceled,
    TreasuryInboundTransferCreated,
    TreasuryInboundTransferFailed,
    TreasuryInboundTransferSucceeded,
    TreasuryOutboundPaymentCanceled,
    TreasuryOutboundPaymentCreated,
    TreasuryOutboundPaymentExpectedArrivalDateUpdated,
    TreasuryOutboundPaymentFailed,
    TreasuryOutboundPaymentPosted,
    TreasuryOutboundPaymentReturned,
    TreasuryOutboundTransferCanceled,
    TreasuryOutboundTransferCreated,
    TreasuryOutboundTransferExpectedArrivalDateUpdated,
    TreasuryOutboundTransferFailed,
    TreasuryOutboundTransferPosted,
    TreasuryOutboundTransferReturned,
    TreasuryReceivedCreditCreated,
    TreasuryReceivedCreditFailed,
    TreasuryReceivedCreditSucceeded,
    TreasuryReceivedDebitCreated,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ClimateSupplier {
    pub id: String,
    pub info_url: String,
    pub livemode: bool,
    pub locations: Vec<ClimateRemovalsLocation>,
    pub name: String,
    pub object: String,
    pub removal_pathway: ClimateSupplierRemovalPathway,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CustomerSources {
    pub data: Vec<PaymentSource>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingCardGooglePay {
    pub eligible: bool,
    pub ineligible_reason: IssuingCardGooglePayIneligibleReason,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsCashappSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Error {
    pub error: ApiErrors,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GelatoIdNumberReportErrorCode {
    IdNumberInsufficientDocumentData,
    IdNumberMismatch,
    IdNumberUnverifiedOther,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LegalEntityJapanAddress {
    pub city: String,
    pub country: String,
    pub line1: String,
    pub line2: String,
    pub postal_code: String,
    pub state: String,
    pub town: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionPhoneNumberCollection {
    pub enabled: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntentPaymentMethodOptionsAcssDebitCurrency {
    Cad,
    Usd,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SourceTypeBancontact {
    pub bank_code: Option<String>,
    pub bank_name: Option<String>,
    pub bic: Option<String>,
    pub iban_last4: Option<String>,
    pub preferred_language: Option<String>,
    pub statement_descriptor: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SubscriptionScheduleConfigurationItem {
    pub billing_thresholds: ReplaceMeWithAnyOfSpec,
    pub metadata: HashMap<String, String>,
    pub plan: ReplaceMeWithAnyOfSpec,
    pub price: ReplaceMeWithAnyOfSpec,
    pub quantity: Option<i64>,
    pub tax_rates: Option<Vec<TaxRate>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SubscriptionsResourcePaymentSettings {
    pub payment_method_options: ReplaceMeWithAnyOfSpec,
    pub payment_method_types: Vec<
        SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem,
    >,
    pub save_default_payment_method: SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GelatoSelfieReport {
    pub document: String,
    pub error: ReplaceMeWithAnyOfSpec,
    pub selfie: String,
    pub status: GelatoSelfieReportStatus,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SourceTypeAlipay {
    pub data_string: Option<String>,
    pub native_url: Option<String>,
    pub statement_descriptor: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardholderPreferredLocalesItem {
    De,
    En,
    Es,
    Fr,
    It,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BillingDetails {
    pub address: ReplaceMeWithAnyOfSpec,
    pub email: String,
    pub name: String,
    pub phone: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InvoicePaymentMethodOptionsUsBankAccount {
    pub financial_connections: Option<
        InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptions,
    >,
    pub verification_method: Option<
        InvoicePaymentMethodOptionsUsBankAccountVerificationMethod,
    >,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalConnectionToken {
    pub location: Option<String>,
    pub object: String,
    pub secret: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodOptionsRevolutPay {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryFinancialAccount {
    pub active_features: Option<Vec<TreasuryFinancialAccountActiveFeaturesItem>>,
    pub balance: TreasuryFinancialAccountsResourceBalance,
    pub country: String,
    pub created: i64,
    pub features: Option<TreasuryFinancialAccountFeatures>,
    pub financial_addresses: Vec<TreasuryFinancialAccountsResourceFinancialAddress>,
    pub id: String,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub object: String,
    pub pending_features: Option<Vec<TreasuryFinancialAccountPendingFeaturesItem>>,
    pub platform_restrictions: Option<ReplaceMeWithAnyOfSpec>,
    pub restricted_features: Option<Vec<TreasuryFinancialAccountRestrictedFeaturesItem>>,
    pub status: TreasuryFinancialAccountStatus,
    pub status_details: TreasuryFinancialAccountsResourceStatusDetails,
    pub supported_currencies: Vec<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PortalFlowsRetention {
    pub coupon_offer: ReplaceMeWithAnyOfSpec,
    pub r#type: PortalFlowsRetentionType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DiscountsResourceDiscountAmount {
    pub amount: i64,
    pub discount: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SourceTypeCard {
    pub address_line1_check: Option<String>,
    pub address_zip_check: Option<String>,
    pub brand: Option<String>,
    pub country: Option<String>,
    pub cvc_check: Option<String>,
    pub description: Option<String>,
    pub dynamic_last4: Option<String>,
    pub exp_month: Option<i64>,
    pub exp_year: Option<i64>,
    pub fingerprint: Option<String>,
    pub funding: Option<String>,
    pub iin: Option<String>,
    pub issuer: Option<String>,
    pub last4: Option<String>,
    pub name: Option<String>,
    pub three_d_secure: Option<String>,
    pub tokenization_method: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentPaymentMethodOptionsSepaDebit {
    pub mandate_options: Option<
        PaymentIntentPaymentMethodOptionsMandateOptionsSepaDebit,
    >,
    pub setup_future_usage: Option<
        PaymentIntentPaymentMethodOptionsSepaDebitSetupFutureUsage,
    >,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingDisputeEvidence {
    pub canceled: Option<IssuingDisputeCanceledEvidence>,
    pub duplicate: Option<IssuingDisputeDuplicateEvidence>,
    pub fraudulent: Option<IssuingDisputeFraudulentEvidence>,
    pub merchandise_not_as_described: Option<
        IssuingDisputeMerchandiseNotAsDescribedEvidence,
    >,
    pub not_received: Option<IssuingDisputeNotReceivedEvidence>,
    pub other: Option<IssuingDisputeOtherEvidence>,
    pub reason: IssuingDisputeEvidenceReason,
    pub service_not_as_described: Option<IssuingDisputeServiceNotAsDescribedEvidence>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalReaderReaderResourceProcessSetupIntentAction {
    pub generated_card: Option<String>,
    pub process_config: Option<TerminalReaderReaderResourceProcessSetupConfig>,
    pub setup_intent: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PortalFlowsFlowSubscriptionUpdateConfirm {
    pub discounts: Vec<PortalFlowsSubscriptionUpdateConfirmDiscount>,
    pub items: Vec<PortalFlowsSubscriptionUpdateConfirmItem>,
    pub subscription: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentNextActionCardAwaitNotification {
    pub charge_attempt_at: i64,
    pub customer_approval_required: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsCardPresentReceipt {
    pub account_type: Option<PaymentMethodDetailsCardPresentReceiptAccountType>,
    pub application_cryptogram: String,
    pub application_preferred_name: String,
    pub authorization_code: String,
    pub authorization_response_code: String,
    pub cardholder_verification_method: String,
    pub dedicated_file_name: String,
    pub terminal_verification_results: String,
    pub transaction_status_information: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetupAttemptPaymentMethodDetailsAcssDebit {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CheckoutIdealPaymentMethodOptions {
    pub setup_future_usage: Option<CheckoutIdealPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsCardWalletVisaCheckout {
    pub billing_address: ReplaceMeWithAnyOfSpec,
    pub email: String,
    pub name: String,
    pub shipping_address: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodInteracPresent {
    pub brand: String,
    pub cardholder_name: String,
    pub country: String,
    pub description: Option<String>,
    pub exp_month: i64,
    pub exp_year: i64,
    pub fingerprint: String,
    pub funding: String,
    pub iin: Option<String>,
    pub issuer: Option<String>,
    pub last4: String,
    pub networks: ReplaceMeWithAnyOfSpec,
    pub preferred_locales: Vec<String>,
    pub read_method: PaymentMethodInteracPresentReadMethod,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentLinksResourceCompletionBehaviorRedirect {
    pub url: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentPagesCheckoutSessionCustomFieldsType {
    Dropdown,
    Numeric,
    Text,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingNetworkTokenWalletProviderReasonCodesItem {
    AccountCardTooNew,
    AccountRecentlyChanged,
    AccountTooNew,
    AccountTooNewSinceLaunch,
    AdditionalDevice,
    DataExpired,
    DeferIdVDecision,
    DeviceRecentlyLost,
    GoodActivityHistory,
    HasSuspendedTokens,
    HighRisk,
    InactiveAccount,
    LongAccountTenure,
    LowAccountScore,
    LowDeviceScore,
    LowPhoneNumberScore,
    NetworkServiceError,
    OutsideHomeTerritory,
    ProvisioningCardholderMismatch,
    ProvisioningDeviceAndCardholderMismatch,
    ProvisioningDeviceMismatch,
    SameDeviceNoPriorAuthentication,
    SameDeviceSuccessfulPriorAuthentication,
    SoftwareUpdate,
    SuspiciousActivity,
    TooManyDifferentCardholders,
    TooManyRecentAttempts,
    TooManyRecentTokens,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaypalSellerProtection {
    pub dispute_categories: Vec<PaypalSellerProtectionDisputeCategoriesItem>,
    pub status: PaypalSellerProtectionStatus,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutCustomerBalancePaymentMethodOptionsFundingType {
    BankTransfer,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinancialConnectionsAccountPermissionsItem {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsKlarnaCaptureMethod {
    Manual,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoicePaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    Business,
    Personal,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodUsBankAccount {
    pub account_holder_type: PaymentMethodUsBankAccountAccountHolderType,
    pub account_type: PaymentMethodUsBankAccountAccountType,
    pub bank_name: String,
    pub financial_connections_account: String,
    pub fingerprint: String,
    pub last4: String,
    pub networks: ReplaceMeWithAnyOfSpec,
    pub routing_number: String,
    pub status_details: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CountrySpecVerificationFieldDetails {
    pub additional: Vec<String>,
    pub minimum: Vec<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutSepaDebitPaymentMethodOptionsSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GelatoVerifiedOutputs {
    pub address: ReplaceMeWithAnyOfSpec,
    pub dob: ReplaceMeWithAnyOfSpec,
    pub first_name: String,
    pub id_number: String,
    pub id_number_type: GelatoVerifiedOutputsIdNumberType,
    pub last_name: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    Company,
    Individual,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutSessionPaymentMethodCollection {
    Always,
    IfRequired,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentLinksResourcePaymentMethodReuseAgreementPosition {
    Auto,
    Hidden,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsAfterpayClearpay {
    pub order_id: String,
    pub reference: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodOptionsCashapp {
    pub capture_method: Option<PaymentMethodOptionsCashappCaptureMethod>,
    pub setup_future_usage: Option<PaymentMethodOptionsCashappSetupFutureUsage>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductResourceLineItemTaxBreakdownTaxabilityReason {
    CustomerExempt,
    NotCollecting,
    NotSubjectToTax,
    NotSupported,
    PortionProductExempt,
    PortionReducedRated,
    PortionStandardRated,
    ProductExempt,
    ProductExemptHoliday,
    ProportionallyRated,
    ReducedRated,
    ReverseCharge,
    StandardRated,
    TaxableBasisReduced,
    ZeroRated,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode {
    Activating,
    CapabilityNotRequested,
    FinancialAccountClosed,
    RejectedOther,
    RejectedUnsupportedBusiness,
    RequirementsPastDue,
    RequirementsPendingVerification,
    RestrictedByPlatform,
    RestrictedOther,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CustomerTaxAutomaticTax {
    Failed,
    NotCollecting,
    Supported,
    UnrecognizedLocation,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardShippingService {
    Express,
    Priority,
    Standard,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsCardWalletGooglePay {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason {
    AlreadyReversed,
    DeadlinePassed,
    NetworkRestricted,
    Other,
    SourceFlowRestricted,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AccountMonthlyEstimatedRevenue {
    pub amount: i64,
    pub currency: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentFlowsInstallmentOptions {
    pub enabled: bool,
    pub plan: Option<PaymentMethodDetailsCardInstallmentsPlan>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentLinksResourcePaymentIntentData {
    pub capture_method: PaymentLinksResourcePaymentIntentDataCaptureMethod,
    pub description: String,
    pub metadata: HashMap<String, String>,
    pub setup_future_usage: PaymentLinksResourcePaymentIntentDataSetupFutureUsage,
    pub statement_descriptor: String,
    pub statement_descriptor_suffix: String,
    pub transfer_group: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryFinancialAccountsResourceBalanceInboundPending {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingDisputeFraudulentEvidence {
    pub additional_documentation: ReplaceMeWithAnyOfSpec,
    pub explanation: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AccountSettings {
    pub bacs_debit_payments: Option<AccountBacsDebitPaymentsSettings>,
    pub branding: AccountBrandingSettings,
    pub card_issuing: Option<AccountCardIssuingSettings>,
    pub card_payments: AccountCardPaymentsSettings,
    pub dashboard: AccountDashboardSettings,
    pub invoices: Option<AccountInvoicesSettings>,
    pub payments: AccountPaymentsSettings,
    pub payouts: Option<AccountPayoutSettings>,
    pub sepa_debit_payments: Option<AccountSepaDebitPaymentsSettings>,
    pub treasury: Option<AccountTreasurySettings>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SubscriptionPaymentMethodOptionsCard {
    pub mandate_options: Option<InvoiceMandateOptionsCard>,
    pub network: SubscriptionPaymentMethodOptionsCardNetwork,
    pub request_three_d_secure: SubscriptionPaymentMethodOptionsCardRequestThreeDSecure,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCode {
    pub hosted_instructions_url: String,
    pub mobile_auth_url: String,
    pub qr_code: PaymentIntentNextActionCashappQrCode,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetupAttemptPaymentMethodDetailsLink {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AccountDashboardSettings {
    pub display_name: String,
    pub timezone: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingDisputeOtherEvidenceProductType {
    Merchandise,
    Service,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortalSubscriptionUpdateProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesTransfers {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FundingInstructionsBankTransferSpeiRecord {
    pub bank_code: String,
    pub bank_name: String,
    pub clabe: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsEpsBank {
    ArzteUndApothekerBank,
    AustrianAnadiBankAg,
    BankAustria,
    BankhausCarlSpangler,
    BankhausSchelhammerUndSchatteraAg,
    BawagPskAg,
    BksBankAg,
    BrullKallmusBankAg,
    BtvVierLanderBank,
    CapitalBankGraweGruppeAg,
    DeutscheBankAg,
    Dolomitenbank,
    EasybankAg,
    ErsteBankUndSparkassen,
    HypoAlpeadriabankInternationalAg,
    HypoBankBurgenlandAktiengesellschaft,
    HypoNoeLbFurNiederosterreichUWien,
    HypoOberosterreichSalzburgSteiermark,
    HypoTirolBankAg,
    HypoVorarlbergBankAg,
    MarchfelderBank,
    OberbankAg,
    RaiffeisenBankengruppeOsterreich,
    SchoellerbankAg,
    SpardaBankWien,
    VolksbankGruppe,
    VolkskreditbankAg,
    VrBankBraunau,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductResourceLineItemTaxBreakdownSourcing {
    Destination,
    Origin,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InvoiceRenderingPdf {
    pub page_size: InvoiceRenderingPdfPageSize,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BillingPortalConfiguration {
    pub active: bool,
    pub application: ReplaceMeWithAnyOfSpec,
    pub business_profile: PortalBusinessProfile,
    pub created: i64,
    pub default_return_url: String,
    pub features: PortalFeatures,
    pub id: String,
    pub is_default: bool,
    pub livemode: bool,
    pub login_page: PortalLoginPage,
    pub metadata: HashMap<String, String>,
    pub object: String,
    pub updated: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingCardShipping {
    pub address: Address,
    pub carrier: IssuingCardShippingCarrier,
    pub customs: ReplaceMeWithAnyOfSpec,
    pub eta: i64,
    pub name: String,
    pub phone_number: String,
    pub require_signature: bool,
    pub service: IssuingCardShippingService,
    pub status: IssuingCardShippingStatus,
    pub tracking_number: String,
    pub tracking_url: String,
    pub r#type: IssuingCardShippingType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod {
    Paper,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct QuotesResourceRecurring {
    pub amount_subtotal: i64,
    pub amount_total: i64,
    pub interval: QuotesResourceRecurringInterval,
    pub interval_count: i64,
    pub total_details: QuotesResourceTotalDetails,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SubscriptionSchedule {
    pub application: ReplaceMeWithAnyOfSpec,
    pub canceled_at: i64,
    pub completed_at: i64,
    pub created: i64,
    pub current_phase: ReplaceMeWithAnyOfSpec,
    pub customer: ReplaceMeWithAnyOfSpec,
    pub default_settings: SubscriptionSchedulesResourceDefaultSettings,
    pub end_behavior: SubscriptionScheduleEndBehavior,
    pub id: String,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub object: String,
    pub phases: Vec<SubscriptionSchedulePhaseConfiguration>,
    pub released_at: i64,
    pub released_subscription: String,
    pub status: SubscriptionScheduleStatus,
    pub subscription: ReplaceMeWithAnyOfSpec,
    pub test_clock: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductResourceLineItemTaxRateDetailsTaxType {
    AmusementTax,
    CommunicationsTax,
    Gst,
    Hst,
    Igst,
    Jct,
    LeaseTax,
    Pst,
    Qst,
    Rst,
    SalesTax,
    Vat,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountActiveFeaturesItem {
    CardIssuing,
    DepositInsurance,
    FinancialAddressesAba,
    InboundTransfersAch,
    IntraStripeFlows,
    OutboundPaymentsAch,
    OutboundPaymentsUsDomesticWire,
    OutboundTransfersAch,
    OutboundTransfersUsDomesticWire,
    RemoteDepositCapture,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LineItemsDiscountAmount {
    pub amount: i64,
    pub discount: Discount,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AccountRequirementsError {
    pub code: AccountRequirementsErrorCode,
    pub reason: String,
    pub requirement: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PortalBusinessProfile {
    pub headline: String,
    pub privacy_policy_url: String,
    pub terms_of_service_url: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub action: String,
    pub id: String,
    pub predicate: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardholderRequirementsDisabledReason {
    Listed,
    RejectedListed,
    RequirementsPastDue,
    UnderReview,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodOptionsIdeal {
    pub setup_future_usage: Option<PaymentMethodOptionsIdealSetupFutureUsage>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceCustomerTaxExempt {
    Exempt,
    None,
    Reverse,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CardAvailablePayoutMethodsItem {
    Instant,
    Standard,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InvoicePaymentMethodOptionsCustomerBalance {
    pub bank_transfer: Option<InvoicePaymentMethodOptionsCustomerBalanceBankTransfer>,
    pub funding_type: InvoicePaymentMethodOptionsCustomerBalanceFundingType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LineItemsTaxAmount {
    pub amount: i64,
    pub rate: TaxRate,
    pub taxability_reason: LineItemsTaxAmountTaxabilityReason,
    pub taxable_amount: i64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutBacsDebitPaymentMethodOptionsSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType {
    Balance,
    FinancialAccount,
    IssuingCard,
    Stripe,
    UsBankAccount,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentLinksResourceRestrictions {
    pub completed_sessions: PaymentLinksResourceCompletedSessions,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails {
    pub balance: Option<
        TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance,
    >,
    pub billing_details: TreasurySharedResourceBillingDetails,
    pub financial_account: Option<ReceivedPaymentMethodDetailsFinancialAccount>,
    pub issuing_card: Option<String>,
    pub r#type: TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType,
    pub us_bank_account: Option<
        TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount,
    >,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardApplePayIneligibleReason {
    MissingAgreement,
    MissingCardholderContact,
    UnsupportedRegion,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus {
    MerchantRejected,
    Successful,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentFlowsPrivatePaymentMethodsAlipay {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethod {
    pub acss_debit: Option<PaymentMethodAcssDebit>,
    pub affirm: Option<PaymentMethodAffirm>,
    pub afterpay_clearpay: Option<PaymentMethodAfterpayClearpay>,
    pub alipay: Option<PaymentFlowsPrivatePaymentMethodsAlipay>,
    pub au_becs_debit: Option<PaymentMethodAuBecsDebit>,
    pub bacs_debit: Option<PaymentMethodBacsDebit>,
    pub bancontact: Option<PaymentMethodBancontact>,
    pub billing_details: BillingDetails,
    pub blik: Option<PaymentMethodBlik>,
    pub boleto: Option<PaymentMethodBoleto>,
    pub card: Option<PaymentMethodCard>,
    pub card_present: Option<PaymentMethodCardPresent>,
    pub cashapp: Option<PaymentMethodCashapp>,
    pub created: i64,
    pub customer: ReplaceMeWithAnyOfSpec,
    pub customer_balance: Option<PaymentMethodCustomerBalance>,
    pub eps: Option<PaymentMethodEps>,
    pub fpx: Option<PaymentMethodFpx>,
    pub giropay: Option<PaymentMethodGiropay>,
    pub grabpay: Option<PaymentMethodGrabpay>,
    pub id: String,
    pub ideal: Option<PaymentMethodIdeal>,
    pub interac_present: Option<PaymentMethodInteracPresent>,
    pub klarna: Option<PaymentMethodKlarna>,
    pub konbini: Option<PaymentMethodKonbini>,
    pub link: Option<PaymentMethodLink>,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub object: String,
    pub oxxo: Option<PaymentMethodOxxo>,
    pub p24: Option<PaymentMethodP24>,
    pub paynow: Option<PaymentMethodPaynow>,
    pub paypal: Option<PaymentMethodPaypal>,
    pub pix: Option<PaymentMethodPix>,
    pub promptpay: Option<PaymentMethodPromptpay>,
    pub radar_options: Option<RadarRadarOptions>,
    pub revolut_pay: Option<PaymentMethodRevolutPay>,
    pub sepa_debit: Option<PaymentMethodSepaDebit>,
    pub sofort: Option<PaymentMethodSofort>,
    pub swish: Option<PaymentMethodSwish>,
    pub r#type: PaymentMethodType,
    pub us_bank_account: Option<PaymentMethodUsBankAccount>,
    pub wechat_pay: Option<PaymentMethodWechatPay>,
    pub zip: Option<PaymentMethodZip>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CheckoutAcssDebitPaymentMethodOptions {
    pub currency: Option<CheckoutAcssDebitPaymentMethodOptionsCurrency>,
    pub mandate_options: Option<CheckoutAcssDebitMandateOptions>,
    pub setup_future_usage: Option<
        CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage,
    >,
    pub verification_method: Option<
        CheckoutAcssDebitPaymentMethodOptionsVerificationMethod,
    >,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ShippingRateFixedAmountCurrencyOptions {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionCustomerDetails {
    pub address: ReplaceMeWithAnyOfSpec,
    pub email: String,
    pub name: String,
    pub phone: String,
    pub tax_exempt: PaymentPagesCheckoutSessionCustomerDetailsTaxExempt,
    pub tax_ids: Vec<PaymentPagesCheckoutSessionTaxId>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityStatus {
    Active,
    Disabled,
    Inactive,
    Pending,
    Unrequested,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeletedDiscount {
    pub checkout_session: String,
    pub coupon: Coupon,
    pub customer: ReplaceMeWithAnyOfSpec,
    pub deleted: bool,
    pub id: String,
    pub invoice: String,
    pub invoice_item: String,
    pub object: String,
    pub promotion_code: ReplaceMeWithAnyOfSpec,
    pub start: i64,
    pub subscription: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InvoiceTransferData {
    pub amount: i64,
    pub destination: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeletedCoupon {
    pub deleted: bool,
    pub id: String,
    pub object: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Level3 {
    pub customer_reference: Option<String>,
    pub line_items: Vec<Level3LineItems>,
    pub merchant_reference: String,
    pub shipping_address_zip: Option<String>,
    pub shipping_amount: Option<i64>,
    pub shipping_from_zip: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RadarReviewResourceLocation {
    pub city: String,
    pub country: String,
    pub latitude: f64,
    pub longitude: f64,
    pub region: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThreeDSecureDetailsChargeResult {
    AttemptAcknowledged,
    Authenticated,
    Exempted,
    Failed,
    NotSupported,
    ProcessingError,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodOptionsInteracPresent {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GelatoIdNumberReport {
    pub dob: ReplaceMeWithAnyOfSpec,
    pub error: ReplaceMeWithAnyOfSpec,
    pub first_name: String,
    pub id_number: String,
    pub id_number_type: GelatoIdNumberReportIdNumberType,
    pub last_name: String,
    pub status: GelatoIdNumberReportStatus,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MandateMultiUse {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PortalFlowsSubscriptionUpdateConfirmDiscount {
    pub coupon: String,
    pub promotion_code: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Period {
    pub end: i64,
    pub start: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsWechat {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentLinksResourceShippingAddressCollection {
    pub allowed_countries: Vec<
        PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem,
    >,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsEps {
    pub bank: PaymentMethodDetailsEpsBank,
    pub verified_name: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinancialConnectionsSessionPrefetchItem {
    Balances,
    Ownership,
    Transactions,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetupAttemptPaymentMethodDetailsBancontact {
    pub bank_code: String,
    pub bank_name: String,
    pub bic: String,
    pub generated_sepa_debit: ReplaceMeWithAnyOfSpec,
    pub generated_sepa_debit_mandate: ReplaceMeWithAnyOfSpec,
    pub iban_last4: String,
    pub preferred_language: SetupAttemptPaymentMethodDetailsBancontactPreferredLanguage,
    pub verified_name: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinancialConnectionsTransactionStatus {
    Pending,
    Posted,
    Void,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalReaderReaderResourceRefundPaymentActionReason {
    Duplicate,
    Fraudulent,
    RequestedByCustomer,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OnlineAcceptance {
    pub ip_address: String,
    pub user_agent: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentNextActionSwishQrCode {
    pub data: Option<String>,
    pub image_url_png: Option<String>,
    pub image_url_svg: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxProductRegistrationsResourceCountryOptions {
    pub ae: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,
    pub at: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub au: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,
    pub be: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub bg: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub ca: Option<TaxProductRegistrationsResourceCountryOptionsCanada>,
    pub ch: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,
    pub cl: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,
    pub co: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,
    pub cy: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub cz: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub de: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub dk: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub ee: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub es: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub fi: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub fr: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub gb: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,
    pub gr: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub hr: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub hu: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub id: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,
    pub ie: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub is: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,
    pub it: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub jp: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,
    pub kr: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,
    pub lt: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub lu: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub lv: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub mt: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub mx: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,
    pub my: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,
    pub nl: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub no: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,
    pub nz: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,
    pub pl: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub pt: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub ro: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub sa: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,
    pub se: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub sg: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,
    pub si: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub sk: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    pub th: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,
    pub tr: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,
    pub us: Option<TaxProductRegistrationsResourceCountryOptionsUnitedStates>,
    pub vn: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,
    pub za: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    None,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeletedBankAccount {
    pub currency: Option<String>,
    pub deleted: bool,
    pub id: String,
    pub object: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CreditNoteReason {
    Duplicate,
    Fraudulent,
    OrderChange,
    ProductUnsatisfactory,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryOutboundTransfersResourceStatusTransitions {
    pub canceled_at: i64,
    pub failed_at: i64,
    pub posted_at: i64,
    pub returned_at: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryFinancialAccountsResourceClosedStatusDetails {
    pub reasons: Vec<TreasuryFinancialAccountsResourceClosedStatusDetailsReasonsItem>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows {
    Restricted,
    Unrestricted,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentPaymentMethodOptionsEps {
    pub setup_future_usage: Option<PaymentIntentPaymentMethodOptionsEpsSetupFutureUsage>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentPaymentMethodOptionsAuBecsDebit {
    pub setup_future_usage: Option<
        PaymentIntentPaymentMethodOptionsAuBecsDebitSetupFutureUsage,
    >,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountUnificationAccountControllerType {
    Account,
    Application,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductResourceCustomerDetailsTaxabilityOverride {
    CustomerExempt,
    None,
    ReverseCharge,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CheckoutKonbiniPaymentMethodOptions {
    pub expires_after_days: i64,
    pub setup_future_usage: Option<CheckoutKonbiniPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Person {
    pub account: Option<String>,
    pub additional_tos_acceptances: Option<PersonAdditionalTosAcceptances>,
    pub address: Option<Address>,
    pub address_kana: Option<ReplaceMeWithAnyOfSpec>,
    pub address_kanji: Option<ReplaceMeWithAnyOfSpec>,
    pub created: i64,
    pub dob: Option<LegalEntityDob>,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub first_name_kana: Option<String>,
    pub first_name_kanji: Option<String>,
    pub full_name_aliases: Option<Vec<String>>,
    pub future_requirements: Option<ReplaceMeWithAnyOfSpec>,
    pub gender: Option<String>,
    pub id: String,
    pub id_number_provided: Option<bool>,
    pub id_number_secondary_provided: Option<bool>,
    pub last_name: Option<String>,
    pub last_name_kana: Option<String>,
    pub last_name_kanji: Option<String>,
    pub maiden_name: Option<String>,
    pub metadata: HashMap<String, String>,
    pub nationality: Option<String>,
    pub object: String,
    pub phone: Option<String>,
    pub political_exposure: Option<PersonPoliticalExposure>,
    pub registered_address: Option<Address>,
    pub relationship: Option<PersonRelationship>,
    pub requirements: Option<ReplaceMeWithAnyOfSpec>,
    pub ssn_last_4_provided: Option<bool>,
    pub verification: Option<LegalEntityPersonVerification>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PriceTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AutomaticTax {
    pub enabled: bool,
    pub liability: ReplaceMeWithAnyOfSpec,
    pub status: AutomaticTaxStatus,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ClimateRemovalsProductsPrice {
    pub amount_fees: i64,
    pub amount_subtotal: i64,
    pub amount_total: i64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    Fixed,
    Maximum,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryReceivedCredit {
    pub amount: i64,
    pub created: i64,
    pub currency: String,
    pub description: String,
    pub failure_code: TreasuryReceivedCreditFailureCode,
    pub financial_account: String,
    pub hosted_regulatory_receipt_url: String,
    pub id: String,
    pub initiating_payment_method_details: TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails,
    pub linked_flows: TreasuryReceivedCreditsResourceLinkedFlows,
    pub livemode: bool,
    pub network: TreasuryReceivedCreditNetwork,
    pub object: String,
    pub reversal_details: ReplaceMeWithAnyOfSpec,
    pub status: TreasuryReceivedCreditStatus,
    pub transaction: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingDisputeMerchandiseNotAsDescribedEvidence {
    pub additional_documentation: ReplaceMeWithAnyOfSpec,
    pub explanation: String,
    pub received_at: i64,
    pub return_description: String,
    pub return_status: IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus,
    pub returned_at: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetupIntentPaymentMethodOptionsAcssDebit {
    pub currency: SetupIntentPaymentMethodOptionsAcssDebitCurrency,
    pub mandate_options: Option<SetupIntentPaymentMethodOptionsMandateOptionsAcssDebit>,
    pub verification_method: Option<
        SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod,
    >,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomaticTaxStatus {
    Complete,
    Failed,
    RequiresLocationInputs,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AccountCapabilityFutureRequirements {
    pub alternatives: Vec<AccountRequirementsAlternative>,
    pub current_deadline: i64,
    pub currently_due: Vec<String>,
    pub disabled_reason: String,
    pub errors: Vec<AccountRequirementsError>,
    pub eventually_due: Vec<String>,
    pub past_due: Vec<String>,
    pub pending_verification: Vec<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryTransactionEntry {
    pub balance_impact: TreasuryTransactionsResourceBalanceImpact,
    pub created: i64,
    pub currency: String,
    pub effective_at: i64,
    pub financial_account: String,
    pub flow: String,
    pub flow_details: ReplaceMeWithAnyOfSpec,
    pub flow_type: TreasuryTransactionEntryFlowType,
    pub id: String,
    pub livemode: bool,
    pub object: String,
    pub transaction: ReplaceMeWithAnyOfSpec,
    pub r#type: TreasuryTransactionEntryType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GelatoSelfieReportErrorCode {
    SelfieDocumentMissingPhoto,
    SelfieFaceMismatch,
    SelfieManipulated,
    SelfieUnverifiedOther,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentPaymentMethodOptionsCardNetwork {
    Amex,
    CartesBancaires,
    Diners,
    Discover,
    EftposAu,
    Interac,
    Jcb,
    Mastercard,
    Unionpay,
    Unknown,
    Visa,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentProcessing {
    pub card: Option<PaymentIntentCardProcessing>,
    pub r#type: PaymentIntentProcessingType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentLink {
    pub active: bool,
    pub after_completion: PaymentLinksResourceAfterCompletion,
    pub allow_promotion_codes: bool,
    pub application: ReplaceMeWithAnyOfSpec,
    pub application_fee_amount: i64,
    pub application_fee_percent: f64,
    pub automatic_tax: PaymentLinksResourceAutomaticTax,
    pub billing_address_collection: PaymentLinkBillingAddressCollection,
    pub consent_collection: ReplaceMeWithAnyOfSpec,
    pub currency: String,
    pub custom_fields: Vec<PaymentLinksResourceCustomFields>,
    pub custom_text: PaymentLinksResourceCustomText,
    pub customer_creation: PaymentLinkCustomerCreation,
    pub id: String,
    pub inactive_message: String,
    pub invoice_creation: ReplaceMeWithAnyOfSpec,
    pub line_items: Option<PaymentLinkLineItems>,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub object: String,
    pub on_behalf_of: ReplaceMeWithAnyOfSpec,
    pub payment_intent_data: ReplaceMeWithAnyOfSpec,
    pub payment_method_collection: PaymentLinkPaymentMethodCollection,
    pub payment_method_types: Vec<PaymentLinkPaymentMethodTypesItem>,
    pub phone_number_collection: PaymentLinksResourcePhoneNumberCollection,
    pub restrictions: ReplaceMeWithAnyOfSpec,
    pub shipping_address_collection: ReplaceMeWithAnyOfSpec,
    pub shipping_options: Vec<PaymentLinksResourceShippingOption>,
    pub submit_type: PaymentLinkSubmitType,
    pub subscription_data: ReplaceMeWithAnyOfSpec,
    pub tax_id_collection: PaymentLinksResourceTaxIdCollection,
    pub transfer_data: ReplaceMeWithAnyOfSpec,
    pub url: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BalanceAmountBySourceType {
    pub bank_account: Option<i64>,
    pub card: Option<i64>,
    pub fpx: Option<i64>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentLinksResourcePaymentMethodReuseAgreement {
    pub position: PaymentLinksResourcePaymentMethodReuseAgreementPosition,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanTiersMode {
    Graduated,
    Volume,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxTransaction {
    pub created: i64,
    pub currency: String,
    pub customer: String,
    pub customer_details: TaxProductResourceCustomerDetails,
    pub id: String,
    pub line_items: TaxTransactionLineItems,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub object: String,
    pub reference: String,
    pub reversal: ReplaceMeWithAnyOfSpec,
    pub shipping_cost: ReplaceMeWithAnyOfSpec,
    pub tax_date: i64,
    pub r#type: TaxTransactionType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesZipPayments {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeletedInvoice {
    pub deleted: bool,
    pub id: String,
    pub object: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesTaxReportingUs1099K {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Item {
    pub amount_discount: i64,
    pub amount_subtotal: i64,
    pub amount_tax: i64,
    pub amount_total: i64,
    pub currency: String,
    pub description: String,
    pub discounts: Option<Vec<LineItemsDiscountAmount>>,
    pub id: String,
    pub object: String,
    pub price: ReplaceMeWithAnyOfSpec,
    pub quantity: i64,
    pub taxes: Option<Vec<LineItemsTaxAmount>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryTransactionsResourceFlowDetails {
    pub credit_reversal: Option<TreasuryCreditReversal>,
    pub debit_reversal: Option<TreasuryDebitReversal>,
    pub inbound_transfer: Option<TreasuryInboundTransfer>,
    pub issuing_authorization: Option<IssuingAuthorization>,
    pub outbound_payment: Option<TreasuryOutboundPayment>,
    pub outbound_transfer: Option<TreasuryOutboundTransfer>,
    pub received_credit: Option<TreasuryReceivedCredit>,
    pub received_debit: Option<TreasuryReceivedDebit>,
    pub r#type: TreasuryTransactionsResourceFlowDetailsType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SourceCodeVerificationFlow {
    pub attempts_remaining: i64,
    pub status: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxTransactionLineItems {
    pub data: Vec<TaxTransactionLineItem>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalReaderStatus {
    Offline,
    Online,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TransferReversals {
    pub data: Vec<TransferReversal>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutboundPaymentsPaymentMethodDetailsType {
    FinancialAccount,
    UsBankAccount,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionTaxIdCollection {
    pub enabled: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortalSubscriptionCancellationReasonOptionsItem {
    CustomerService,
    LowQuality,
    MissingFeatures,
    Other,
    SwitchedService,
    TooComplex,
    TooExpensive,
    Unused,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsWechatPay {
    pub fingerprint: String,
    pub transaction_id: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ReportingReportType {
    pub data_available_end: i64,
    pub data_available_start: i64,
    pub default_columns: Vec<String>,
    pub id: String,
    pub livemode: bool,
    pub name: String,
    pub object: String,
    pub updated: i64,
    pub version: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SecretServiceResourceScope {
    pub r#type: SecretServiceResourceScopeType,
    pub user: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SourceTypeGiropay {
    pub bank_code: Option<String>,
    pub bank_name: Option<String>,
    pub bic: Option<String>,
    pub statement_descriptor: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworksItem {
    Ach,
    UsDomesticWire,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GelatoReportDocumentOptionsAllowedTypesItem {
    DrivingLicense,
    IdCard,
    Passport,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CheckoutRevolutPayPaymentMethodOptions {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Account {
    pub business_profile: Option<ReplaceMeWithAnyOfSpec>,
    pub business_type: Option<AccountBusinessType>,
    pub capabilities: Option<AccountCapabilities>,
    pub charges_enabled: Option<bool>,
    pub company: Option<LegalEntityCompany>,
    pub controller: Option<AccountUnificationAccountController>,
    pub country: Option<String>,
    pub created: Option<i64>,
    pub default_currency: Option<String>,
    pub details_submitted: Option<bool>,
    pub email: Option<String>,
    pub external_accounts: Option<AccountExternalAccounts>,
    pub future_requirements: Option<AccountFutureRequirements>,
    pub id: String,
    pub individual: Option<Person>,
    pub metadata: HashMap<String, String>,
    pub object: String,
    pub payouts_enabled: Option<bool>,
    pub requirements: Option<AccountRequirements>,
    pub settings: Option<ReplaceMeWithAnyOfSpec>,
    pub tos_acceptance: Option<AccountTosAcceptance>,
    pub r#type: Option<AccountType>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ConnectEmbeddedBaseConfigClaim {
    pub enabled: bool,
    pub features: ConnectEmbeddedBaseFeatures,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FundingInstructionsBankTransferFinancialAddressSupportedNetworksItem {
    Ach,
    Bacs,
    DomesticWireUs,
    Fps,
    Sepa,
    Spei,
    Swift,
    Zengin,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentNextActionWechatPayRedirectToAndroidApp {
    pub app_id: String,
    pub nonce_str: String,
    pub package: String,
    pub partner_id: String,
    pub prepay_id: String,
    pub sign: String,
    pub timestamp: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodCardWalletVisaCheckout {
    pub billing_address: ReplaceMeWithAnyOfSpec,
    pub email: String,
    pub name: String,
    pub shipping_address: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GelatoSessionDocumentOptions {
    pub allowed_types: Option<Vec<GelatoSessionDocumentOptionsAllowedTypesItem>>,
    pub require_id_number: Option<bool>,
    pub require_live_capture: Option<bool>,
    pub require_matching_selfie: Option<bool>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ShippingRateDeliveryEstimateBound {
    pub unit: ShippingRateDeliveryEstimateBoundUnit,
    pub value: i64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShippingRateTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CheckoutKlarnaPaymentMethodOptions {
    pub setup_future_usage: Option<CheckoutKlarnaPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardholderAuthorizationControlsAllowedCategoriesItem {
    AcRefrigerationRepair,
    AccountingBookkeepingServices,
    AdvertisingServices,
    AgriculturalCooperative,
    AirlinesAirCarriers,
    AirportsFlyingFields,
    AmbulanceServices,
    AmusementParksCarnivals,
    AntiqueReproductions,
    AntiqueShops,
    Aquariums,
    ArchitecturalSurveyingServices,
    ArtDealersAndGalleries,
    ArtistsSupplyAndCraftShops,
    AutoAndHomeSupplyStores,
    AutoBodyRepairShops,
    AutoPaintShops,
    AutoServiceShops,
    AutomatedCashDisburse,
    AutomatedFuelDispensers,
    AutomobileAssociations,
    AutomotivePartsAndAccessoriesStores,
    AutomotiveTireStores,
    BailAndBondPayments,
    Bakeries,
    BandsOrchestras,
    BarberAndBeautyShops,
    BettingCasinoGambling,
    BicycleShops,
    BilliardPoolEstablishments,
    BoatDealers,
    BoatRentalsAndLeases,
    BookStores,
    BooksPeriodicalsAndNewspapers,
    BowlingAlleys,
    BusLines,
    BusinessSecretarialSchools,
    BuyingShoppingServices,
    CableSatelliteAndOtherPayTelevisionAndRadio,
    CameraAndPhotographicSupplyStores,
    CandyNutAndConfectioneryStores,
    CarAndTruckDealersNewUsed,
    CarAndTruckDealersUsedOnly,
    CarRentalAgencies,
    CarWashes,
    CarpentryServices,
    CarpetUpholsteryCleaning,
    Caterers,
    CharitableAndSocialServiceOrganizationsFundraising,
    ChemicalsAndAlliedProducts,
    ChildCareServices,
    ChildrensAndInfantsWearStores,
    ChiropodistsPodiatrists,
    Chiropractors,
    CigarStoresAndStands,
    CivicSocialFraternalAssociations,
    CleaningAndMaintenance,
    ClothingRental,
    CollegesUniversities,
    CommercialEquipment,
    CommercialFootwear,
    CommercialPhotographyArtAndGraphics,
    CommuterTransportAndFerries,
    ComputerNetworkServices,
    ComputerProgramming,
    ComputerRepair,
    ComputerSoftwareStores,
    ComputersPeripheralsAndSoftware,
    ConcreteWorkServices,
    ConstructionMaterials,
    ConsultingPublicRelations,
    CorrespondenceSchools,
    CosmeticStores,
    CounselingServices,
    CountryClubs,
    CourierServices,
    CourtCosts,
    CreditReportingAgencies,
    CruiseLines,
    DairyProductsStores,
    DanceHallStudiosSchools,
    DatingEscortServices,
    DentistsOrthodontists,
    DepartmentStores,
    DetectiveAgencies,
    DigitalGoodsApplications,
    DigitalGoodsGames,
    DigitalGoodsLargeVolume,
    DigitalGoodsMedia,
    DirectMarketingCatalogMerchant,
    DirectMarketingCombinationCatalogAndRetailMerchant,
    DirectMarketingInboundTelemarketing,
    DirectMarketingInsuranceServices,
    DirectMarketingOther,
    DirectMarketingOutboundTelemarketing,
    DirectMarketingSubscription,
    DirectMarketingTravel,
    DiscountStores,
    Doctors,
    DoorToDoorSales,
    DraperyWindowCoveringAndUpholsteryStores,
    DrinkingPlaces,
    DrugStoresAndPharmacies,
    DrugsDrugProprietariesAndDruggistSundries,
    DryCleaners,
    DurableGoods,
    DutyFreeStores,
    EatingPlacesRestaurants,
    EducationalServices,
    ElectricRazorStores,
    ElectricVehicleCharging,
    ElectricalPartsAndEquipment,
    ElectricalServices,
    ElectronicsRepairShops,
    ElectronicsStores,
    ElementarySecondarySchools,
    EmergencyServicesGcasVisaUseOnly,
    EmploymentTempAgencies,
    EquipmentRental,
    ExterminatingServices,
    FamilyClothingStores,
    FastFoodRestaurants,
    FinancialInstitutions,
    FinesGovernmentAdministrativeEntities,
    FireplaceFireplaceScreensAndAccessoriesStores,
    FloorCoveringStores,
    Florists,
    FloristsSuppliesNurseryStockAndFlowers,
    FreezerAndLockerMeatProvisioners,
    FuelDealersNonAutomotive,
    FuneralServicesCrematories,
    FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances,
    FurnitureRepairRefinishing,
    FurriersAndFurShops,
    GeneralServices,
    GiftCardNoveltyAndSouvenirShops,
    GlassPaintAndWallpaperStores,
    GlasswareCrystalStores,
    GolfCoursesPublic,
    GovernmentLicensedHorseDogRacingUsRegionOnly,
    GovernmentLicensedOnlineCasionsOnlineGamblingUsRegionOnly,
    GovernmentOwnedLotteriesNonUsRegion,
    GovernmentOwnedLotteriesUsRegionOnly,
    GovernmentServices,
    GroceryStoresSupermarkets,
    HardwareEquipmentAndSupplies,
    HardwareStores,
    HealthAndBeautySpas,
    HearingAidsSalesAndSupplies,
    HeatingPlumbingAC,
    HobbyToyAndGameShops,
    HomeSupplyWarehouseStores,
    Hospitals,
    HotelsMotelsAndResorts,
    HouseholdApplianceStores,
    IndustrialSupplies,
    InformationRetrievalServices,
    InsuranceDefault,
    InsuranceUnderwritingPremiums,
    IntraCompanyPurchases,
    JewelryStoresWatchesClocksAndSilverwareStores,
    LandscapingServices,
    Laundries,
    LaundryCleaningServices,
    LegalServicesAttorneys,
    LuggageAndLeatherGoodsStores,
    LumberBuildingMaterialsStores,
    ManualCashDisburse,
    MarinasServiceAndSupplies,
    Marketplaces,
    MasonryStoneworkAndPlaster,
    MassageParlors,
    MedicalAndDentalLabs,
    MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies,
    MedicalServices,
    MembershipOrganizations,
    MensAndBoysClothingAndAccessoriesStores,
    MensWomensClothingStores,
    MetalServiceCenters,
    Miscellaneous,
    MiscellaneousApparelAndAccessoryShops,
    MiscellaneousAutoDealers,
    MiscellaneousBusinessServices,
    MiscellaneousFoodStores,
    MiscellaneousGeneralMerchandise,
    MiscellaneousGeneralServices,
    MiscellaneousHomeFurnishingSpecialtyStores,
    MiscellaneousPublishingAndPrinting,
    MiscellaneousRecreationServices,
    MiscellaneousRepairShops,
    MiscellaneousSpecialtyRetail,
    MobileHomeDealers,
    MotionPictureTheaters,
    MotorFreightCarriersAndTrucking,
    MotorHomesDealers,
    MotorVehicleSuppliesAndNewParts,
    MotorcycleShopsAndDealers,
    MotorcycleShopsDealers,
    MusicStoresMusicalInstrumentsPianosAndSheetMusic,
    NewsDealersAndNewsstands,
    NonFiMoneyOrders,
    NonFiStoredValueCardPurchaseLoad,
    NondurableGoods,
    NurseriesLawnAndGardenSupplyStores,
    NursingPersonalCare,
    OfficeAndCommercialFurniture,
    OpticiansEyeglasses,
    OptometristsOphthalmologist,
    OrthopedicGoodsProstheticDevices,
    Osteopaths,
    PackageStoresBeerWineAndLiquor,
    PaintsVarnishesAndSupplies,
    ParkingLotsGarages,
    PassengerRailways,
    PawnShops,
    PetShopsPetFoodAndSupplies,
    PetroleumAndPetroleumProducts,
    PhotoDeveloping,
    PhotographicPhotocopyMicrofilmEquipmentAndSupplies,
    PhotographicStudios,
    PictureVideoProduction,
    PieceGoodsNotionsAndOtherDryGoods,
    PlumbingHeatingEquipmentAndSupplies,
    PoliticalOrganizations,
    PostalServicesGovernmentOnly,
    PreciousStonesAndMetalsWatchesAndJewelry,
    ProfessionalServices,
    PublicWarehousingAndStorage,
    QuickCopyReproAndBlueprint,
    Railroads,
    RealEstateAgentsAndManagersRentals,
    RecordStores,
    RecreationalVehicleRentals,
    ReligiousGoodsStores,
    ReligiousOrganizations,
    RoofingSidingSheetMetal,
    SecretarialSupportServices,
    SecurityBrokersDealers,
    ServiceStations,
    SewingNeedleworkFabricAndPieceGoodsStores,
    ShoeRepairHatCleaning,
    ShoeStores,
    SmallApplianceRepair,
    SnowmobileDealers,
    SpecialTradeServices,
    SpecialtyCleaning,
    SportingGoodsStores,
    SportingRecreationCamps,
    SportsAndRidingApparelStores,
    SportsClubsFields,
    StampAndCoinStores,
    StationaryOfficeSuppliesPrintingAndWritingPaper,
    StationeryStoresOfficeAndSchoolSupplyStores,
    SwimmingPoolsSales,
    TUiTravelGermany,
    TailorsAlterations,
    TaxPaymentsGovernmentAgencies,
    TaxPreparationServices,
    TaxicabsLimousines,
    TelecommunicationEquipmentAndTelephoneSales,
    TelecommunicationServices,
    TelegraphServices,
    TentAndAwningShops,
    TestingLaboratories,
    TheatricalTicketAgencies,
    Timeshares,
    TireRetreadingAndRepair,
    TollsBridgeFees,
    TouristAttractionsAndExhibits,
    TowingServices,
    TrailerParksCampgrounds,
    TransportationServices,
    TravelAgenciesTourOperators,
    TruckStopIteration,
    TruckUtilityTrailerRentals,
    TypesettingPlateMakingAndRelatedServices,
    TypewriterStores,
    USFederalGovernmentAgenciesOrDepartments,
    UniformsCommercialClothing,
    UsedMerchandiseAndSecondhandStores,
    Utilities,
    VarietyStores,
    VeterinaryServices,
    VideoAmusementGameSupplies,
    VideoGameArcades,
    VideoTapeRentalStores,
    VocationalTradeSchools,
    WatchJewelryRepair,
    WeldingRepair,
    WholesaleClubs,
    WigAndToupeeStores,
    WiresMoneyOrders,
    WomensAccessoryAndSpecialtyShops,
    WomensReadyToWearStores,
    WreckingAndSalvageYards,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CancellationDetailsReason {
    CancellationRequested,
    PaymentDisputed,
    PaymentFailed,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentLinksResourceConsentCollection {
    pub payment_method_reuse_agreement: ReplaceMeWithAnyOfSpec,
    pub promotions: PaymentLinksResourceConsentCollectionPromotions,
    pub terms_of_service: PaymentLinksResourceConsentCollectionTermsOfService,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeletedTaxId {
    pub deleted: bool,
    pub id: String,
    pub object: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentPaymentMethodOptionsSwish {
    pub reference: Option<String>,
    pub setup_future_usage: Option<
        PaymentIntentPaymentMethodOptionsSwishSetupFutureUsage,
    >,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentPaymentMethodOptionsLinkSetupFutureUsage {
    None,
    OffSession,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentLinkSubmitType {
    Auto,
    Book,
    Donate,
    Pay,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodOptionsPaynow {
    pub setup_future_usage: Option<PaymentMethodOptionsPaynowSetupFutureUsage>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductResourceJurisdictionLevel {
    City,
    Country,
    County,
    District,
    State,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaxRateTaxType {
    AmusementTax,
    CommunicationsTax,
    Gst,
    Hst,
    Igst,
    Jct,
    LeaseTax,
    Pst,
    Qst,
    Rst,
    SalesTax,
    ServiceTax,
    Vat,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingTransactionPurchaseDetails {
    pub flight: ReplaceMeWithAnyOfSpec,
    pub fuel: ReplaceMeWithAnyOfSpec,
    pub lodging: ReplaceMeWithAnyOfSpec,
    pub receipt: Vec<IssuingTransactionReceiptData>,
    pub reference: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApiErrorsType {
    ApiError,
    CardError,
    IdempotencyError,
    InvalidRequestError,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingNetworkTokenWalletProviderCardNumberSource {
    App,
    Manual,
    OnFile,
    Other,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodOptionsP24 {
    pub setup_future_usage: Option<PaymentMethodOptionsP24SetupFutureUsage>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesP24Payments {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentFlowsAutomaticPaymentMethodsPaymentIntent {
    pub allow_redirects: Option<
        PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects,
    >,
    pub enabled: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct QuoteLineItems {
    pub data: Vec<Item>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsCardInstallmentsPlanType {
    FixedCount,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    Amounts,
    DescriptorCode,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutAlipayPaymentMethodOptionsSetupFutureUsage {
    None,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentNextActionWechatPayDisplayQrCode {
    pub data: String,
    pub hosted_instructions_url: String,
    pub image_data_url: String,
    pub image_url_png: String,
    pub image_url_svg: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryFinancialAccountsResourceAbaRecord {
    pub account_holder_name: String,
    pub account_number: String,
    pub account_number_last4: String,
    pub bank_name: String,
    pub routing_number: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitions {
    pub canceled_at: i64,
    pub failed_at: i64,
    pub posted_at: i64,
    pub returned_at: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingAuthorizationAmountDetails {
    pub atm_fee: i64,
    pub cashback_amount: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionTotalDetails {
    pub amount_discount: i64,
    pub amount_shipping: i64,
    pub amount_tax: i64,
    pub breakdown: Option<PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsGrabpay {
    pub transaction_id: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentNextActionWechatPayRedirectToIosApp {
    pub native_url: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsCustomerBalanceEuBankAccountCountry {
    BE,
    DE,
    ES,
    FR,
    IE,
    NL,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryTransaction {
    pub amount: i64,
    pub balance_impact: TreasuryTransactionsResourceBalanceImpact,
    pub created: i64,
    pub currency: String,
    pub description: String,
    pub entries: TreasuryTransactionEntries,
    pub financial_account: String,
    pub flow: String,
    pub flow_details: ReplaceMeWithAnyOfSpec,
    pub flow_type: TreasuryTransactionFlowType,
    pub id: String,
    pub livemode: bool,
    pub object: String,
    pub status: TreasuryTransactionStatus,
    pub status_transitions: TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitions,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentPaymentMethodOptionsUsBankAccountPreferredSettlementSpeed {
    Fastest,
    Standard,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BankConnectionsResourceBalance {
    pub as_of: i64,
    pub cash: Option<BankConnectionsResourceBalanceApiResourceCashBalance>,
    pub credit: Option<BankConnectionsResourceBalanceApiResourceCreditBalance>,
    pub current: BankConnectionsResourceBalanceCurrent,
    pub r#type: BankConnectionsResourceBalanceType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingTransactionTreasury {
    pub received_credit: String,
    pub received_debit: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SourceOrderItem {
    pub amount: i64,
    pub currency: String,
    pub description: String,
    pub parent: String,
    pub quantity: Option<i64>,
    pub r#type: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentLinksResourcePaymentIntentDataCaptureMethod {
    Automatic,
    AutomaticAsync,
    Manual,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceTransactionType {
    AchCreditTransfer,
    AchDebit,
    Alipay,
    Bancontact,
    Card,
    CardPresent,
    Eps,
    Giropay,
    Ideal,
    Klarna,
    Multibanco,
    P24,
    SepaDebit,
    Sofort,
    ThreeDSecure,
    Wechat,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AccountCapabilities {
    pub acss_debit_payments: Option<AccountCapabilitiesAcssDebitPayments>,
    pub affirm_payments: Option<AccountCapabilitiesAffirmPayments>,
    pub afterpay_clearpay_payments: Option<AccountCapabilitiesAfterpayClearpayPayments>,
    pub au_becs_debit_payments: Option<AccountCapabilitiesAuBecsDebitPayments>,
    pub bacs_debit_payments: Option<AccountCapabilitiesBacsDebitPayments>,
    pub bancontact_payments: Option<AccountCapabilitiesBancontactPayments>,
    pub bank_transfer_payments: Option<AccountCapabilitiesBankTransferPayments>,
    pub blik_payments: Option<AccountCapabilitiesBlikPayments>,
    pub boleto_payments: Option<AccountCapabilitiesBoletoPayments>,
    pub card_issuing: Option<AccountCapabilitiesCardIssuing>,
    pub card_payments: Option<AccountCapabilitiesCardPayments>,
    pub cartes_bancaires_payments: Option<AccountCapabilitiesCartesBancairesPayments>,
    pub cashapp_payments: Option<AccountCapabilitiesCashappPayments>,
    pub eps_payments: Option<AccountCapabilitiesEpsPayments>,
    pub fpx_payments: Option<AccountCapabilitiesFpxPayments>,
    pub giropay_payments: Option<AccountCapabilitiesGiropayPayments>,
    pub grabpay_payments: Option<AccountCapabilitiesGrabpayPayments>,
    pub ideal_payments: Option<AccountCapabilitiesIdealPayments>,
    pub india_international_payments: Option<
        AccountCapabilitiesIndiaInternationalPayments,
    >,
    pub jcb_payments: Option<AccountCapabilitiesJcbPayments>,
    pub klarna_payments: Option<AccountCapabilitiesKlarnaPayments>,
    pub konbini_payments: Option<AccountCapabilitiesKonbiniPayments>,
    pub legacy_payments: Option<AccountCapabilitiesLegacyPayments>,
    pub link_payments: Option<AccountCapabilitiesLinkPayments>,
    pub oxxo_payments: Option<AccountCapabilitiesOxxoPayments>,
    pub p24_payments: Option<AccountCapabilitiesP24Payments>,
    pub paynow_payments: Option<AccountCapabilitiesPaynowPayments>,
    pub promptpay_payments: Option<AccountCapabilitiesPromptpayPayments>,
    pub revolut_pay_payments: Option<AccountCapabilitiesRevolutPayPayments>,
    pub sepa_debit_payments: Option<AccountCapabilitiesSepaDebitPayments>,
    pub sofort_payments: Option<AccountCapabilitiesSofortPayments>,
    pub swish_payments: Option<AccountCapabilitiesSwishPayments>,
    pub tax_reporting_us_1099_k: Option<AccountCapabilitiesTaxReportingUs1099K>,
    pub tax_reporting_us_1099_misc: Option<AccountCapabilitiesTaxReportingUs1099Misc>,
    pub transfers: Option<AccountCapabilitiesTransfers>,
    pub treasury: Option<AccountCapabilitiesTreasury>,
    pub us_bank_account_ach_payments: Option<
        AccountCapabilitiesUsBankAccountAchPayments,
    >,
    pub zip_payments: Option<AccountCapabilitiesZipPayments>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuoteCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionPaymentMethodOptionsCardNetwork {
    Amex,
    CartesBancaires,
    Diners,
    Discover,
    EftposAu,
    Interac,
    Jcb,
    Mastercard,
    Unionpay,
    Unknown,
    Visa,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesTreasury {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CheckoutAfterpayClearpayPaymentMethodOptions {
    pub setup_future_usage: Option<
        CheckoutAfterpayClearpayPaymentMethodOptionsSetupFutureUsage,
    >,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsPermissionsItem {
    Balances,
    PaymentMethod,
    Transactions,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsFpx {
    pub account_holder_type: PaymentMethodDetailsFpxAccountHolderType,
    pub bank: PaymentMethodDetailsFpxBank,
    pub transaction_id: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodUsBankAccountAccountType {
    Checking,
    Savings,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryReceivedDebitFailureCode {
    AccountClosed,
    AccountFrozen,
    InsufficientFunds,
    Other,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct WebhookEndpoint {
    pub api_version: String,
    pub application: String,
    pub created: i64,
    pub description: String,
    pub enabled_events: Vec<String>,
    pub id: String,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub object: String,
    pub secret: Option<String>,
    pub status: String,
    pub url: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesCartesBancairesPayments {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingNetworkTokenMastercard {
    pub card_reference_id: Option<String>,
    pub token_reference_id: String,
    pub token_requestor_id: String,
    pub token_requestor_name: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MandateAuBecsDebit {
    pub url: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodOptionsUsBankAccountMandateOptions {
    pub collection_method: Option<
        PaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod,
    >,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentPagesCheckoutSessionConsentPromotions {
    OptIn,
    OptOut,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PriceType {
    OneTime,
    Recurring,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ShippingRate {
    pub active: bool,
    pub created: i64,
    pub delivery_estimate: ReplaceMeWithAnyOfSpec,
    pub display_name: String,
    pub fixed_amount: Option<ShippingRateFixedAmount>,
    pub id: String,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub object: String,
    pub tax_behavior: ShippingRateTaxBehavior,
    pub tax_code: ReplaceMeWithAnyOfSpec,
    pub r#type: ShippingRateType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceType {
    AchCreditTransfer,
    AchDebit,
    AcssDebit,
    Alipay,
    AuBecsDebit,
    Bancontact,
    Card,
    CardPresent,
    Eps,
    Giropay,
    Ideal,
    Klarna,
    Multibanco,
    P24,
    SepaCreditTransfer,
    SepaDebit,
    Sofort,
    ThreeDSecure,
    Wechat,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingAuthorization {
    pub amount: i64,
    pub amount_details: ReplaceMeWithAnyOfSpec,
    pub approved: bool,
    pub authorization_method: IssuingAuthorizationAuthorizationMethod,
    pub balance_transactions: Vec<BalanceTransaction>,
    pub card: IssuingCard,
    pub cardholder: ReplaceMeWithAnyOfSpec,
    pub created: i64,
    pub currency: String,
    pub id: String,
    pub livemode: bool,
    pub merchant_amount: i64,
    pub merchant_currency: String,
    pub merchant_data: IssuingAuthorizationMerchantData,
    pub metadata: HashMap<String, String>,
    pub network_data: ReplaceMeWithAnyOfSpec,
    pub object: String,
    pub pending_request: ReplaceMeWithAnyOfSpec,
    pub request_history: Vec<IssuingAuthorizationRequest>,
    pub status: IssuingAuthorizationStatus,
    pub token: Option<ReplaceMeWithAnyOfSpec>,
    pub transactions: Vec<IssuingTransaction>,
    pub treasury: Option<ReplaceMeWithAnyOfSpec>,
    pub verification_data: IssuingAuthorizationVerificationData,
    pub wallet: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SourceReceiverFlow {
    pub address: String,
    pub amount_charged: i64,
    pub amount_received: i64,
    pub amount_returned: i64,
    pub refund_attributes_method: String,
    pub refund_attributes_status: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxId {
    pub country: String,
    pub created: i64,
    pub customer: ReplaceMeWithAnyOfSpec,
    pub id: String,
    pub livemode: bool,
    pub object: String,
    pub owner: Option<ReplaceMeWithAnyOfSpec>,
    pub r#type: TaxIdType,
    pub value: String,
    pub verification: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryReceivedCreditsResourceLinkedFlows {
    pub credit_reversal: String,
    pub issuing_authorization: String,
    pub issuing_transaction: String,
    pub source_flow: String,
    pub source_flow_details: ReplaceMeWithAnyOfSpec,
    pub source_flow_type: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IdentityVerificationReport {
    pub created: i64,
    pub document: Option<GelatoDocumentReport>,
    pub id: String,
    pub id_number: Option<GelatoIdNumberReport>,
    pub livemode: bool,
    pub object: String,
    pub options: Option<GelatoVerificationReportOptions>,
    pub selfie: Option<GelatoSelfieReport>,
    pub r#type: Option<IdentityVerificationReportType>,
    pub verification_session: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardholderType {
    Company,
    Individual,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ShippingRateDeliveryEstimate {
    pub maximum: ReplaceMeWithAnyOfSpec,
    pub minimum: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraft {
    pub balance_transaction: ReplaceMeWithAnyOfSpec,
    pub linked_transaction: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CustomerSessionResourceComponentsResourcePricingTable {
    pub enabled: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesAcssDebitPayments {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodCardWalletAmexExpressCheckout {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AppsSecret {
    pub created: i64,
    pub deleted: Option<bool>,
    pub expires_at: i64,
    pub id: String,
    pub livemode: bool,
    pub name: String,
    pub object: String,
    pub payload: String,
    pub scope: SecretServiceResourceScope,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupAttemptPaymentMethodDetailsIdealBank {
    AbnAmro,
    AsnBank,
    Bunq,
    Handelsbanken,
    Ing,
    Knab,
    Moneyou,
    N26,
    Nn,
    Rabobank,
    Regiobank,
    Revolut,
    SnsBank,
    TriodosBank,
    VanLanschot,
    Yoursafe,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeletedSubscriptionItem {
    pub deleted: bool,
    pub id: String,
    pub object: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentLinksResourceAfterCompletion {
    pub hosted_confirmation: Option<
        PaymentLinksResourceCompletionBehaviorConfirmationPage,
    >,
    pub redirect: Option<PaymentLinksResourceCompletionBehaviorRedirect>,
    pub r#type: PaymentLinksResourceAfterCompletionType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodAfterpayClearpay {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodCardWalletType {
    AmexExpressCheckout,
    ApplePay,
    GooglePay,
    Link,
    Masterpass,
    SamsungPay,
    VisaCheckout,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeletedWebhookEndpoint {
    pub deleted: bool,
    pub id: String,
    pub object: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub application: ReplaceMeWithAnyOfSpec,
    pub application_fee_percent: f64,
    pub automatic_tax: SubscriptionAutomaticTax,
    pub billing_cycle_anchor: i64,
    pub billing_cycle_anchor_config: ReplaceMeWithAnyOfSpec,
    pub billing_thresholds: ReplaceMeWithAnyOfSpec,
    pub cancel_at: i64,
    pub cancel_at_period_end: bool,
    pub canceled_at: i64,
    pub cancellation_details: ReplaceMeWithAnyOfSpec,
    pub collection_method: SubscriptionCollectionMethod,
    pub created: i64,
    pub currency: String,
    pub current_period_end: i64,
    pub current_period_start: i64,
    pub customer: ReplaceMeWithAnyOfSpec,
    pub days_until_due: i64,
    pub default_payment_method: ReplaceMeWithAnyOfSpec,
    pub default_source: ReplaceMeWithAnyOfSpec,
    pub default_tax_rates: Option<Vec<TaxRate>>,
    pub description: String,
    pub discount: ReplaceMeWithAnyOfSpec,
    pub ended_at: i64,
    pub id: String,
    pub items: SubscriptionItems,
    pub latest_invoice: ReplaceMeWithAnyOfSpec,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub next_pending_invoice_item_invoice: i64,
    pub object: String,
    pub on_behalf_of: ReplaceMeWithAnyOfSpec,
    pub pause_collection: ReplaceMeWithAnyOfSpec,
    pub payment_settings: ReplaceMeWithAnyOfSpec,
    pub pending_invoice_item_interval: ReplaceMeWithAnyOfSpec,
    pub pending_setup_intent: ReplaceMeWithAnyOfSpec,
    pub pending_update: ReplaceMeWithAnyOfSpec,
    pub schedule: ReplaceMeWithAnyOfSpec,
    pub start_date: i64,
    pub status: SubscriptionStatus,
    pub test_clock: ReplaceMeWithAnyOfSpec,
    pub transfer_data: ReplaceMeWithAnyOfSpec,
    pub trial_end: i64,
    pub trial_settings: ReplaceMeWithAnyOfSpec,
    pub trial_start: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingAuthorizationMerchantData {
    pub category: String,
    pub category_code: String,
    pub city: String,
    pub country: String,
    pub name: String,
    pub network_id: String,
    pub postal_code: String,
    pub state: String,
    pub terminal_id: String,
    pub url: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsP24Bank {
    AliorBank,
    BankMillennium,
    BankNowyBfgSa,
    BankPekaoSa,
    BankiSpbdzielcze,
    Blik,
    BnpParibas,
    Boz,
    CitiHandlowy,
    CreditAgricole,
    Envelobank,
    EtransferPocztowy24,
    GetinBank,
    Ideabank,
    Ing,
    Inteligo,
    MbankMtransfer,
    NestPrzelew,
    NoblePay,
    PbacZIpko,
    PlusBank,
    SantanderPrzelew24,
    TmobileUsbugiBankowe,
    ToyotaBank,
    VolkswagenBank,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountBusinessType {
    Company,
    GovernmentEntity,
    Individual,
    NonProfit,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeletedApplePayDomain {
    pub deleted: bool,
    pub id: String,
    pub object: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsBancontactPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodOptionsOxxo {
    pub expires_after_days: i64,
    pub setup_future_usage: Option<PaymentMethodOptionsOxxoSetupFutureUsage>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RefundDestinationDetailsGeneric {
    pub reference: String,
    pub reference_status: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AccountTermsOfService {
    pub date: i64,
    pub ip: String,
    pub user_agent: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MandateCashapp {}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BalanceTransaction {
    pub amount: i64,
    pub available_on: i64,
    pub created: i64,
    pub currency: String,
    pub description: String,
    pub exchange_rate: f64,
    pub fee: i64,
    pub fee_details: Vec<Fee>,
    pub id: String,
    pub net: i64,
    pub object: String,
    pub reporting_category: String,
    pub source: ReplaceMeWithAnyOfSpec,
    pub status: String,
    pub r#type: BalanceTransactionType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount {
    pub bank_name: String,
    pub last4: String,
    pub routing_number: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BankConnectionsResourceBalanceApiResourceCreditBalance {
    pub used: BankConnectionsResourceBalanceApiResourceCreditBalanceUsed,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutboundPaymentsPaymentMethodDetailsUsBankAccountAccountHolderType {
    Company,
    Individual,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutBancontactPaymentMethodOptionsSetupFutureUsage {
    None,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingCardShippingCustoms {
    pub eori_number: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodOptionsBancontact {
    pub preferred_language: PaymentMethodOptionsBancontactPreferredLanguage,
    pub setup_future_usage: Option<PaymentMethodOptionsBancontactSetupFutureUsage>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BankAccountAvailablePayoutMethodsItem {
    Instant,
    Standard,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryFinancialAccountsResourceFinancialAddress {
    pub aba: Option<TreasuryFinancialAccountsResourceAbaRecord>,
    pub supported_networks: Option<
        Vec<TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworksItem>,
    >,
    pub r#type: TreasuryFinancialAccountsResourceFinancialAddressType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown {
    pub discounts: Vec<LineItemsDiscountAmount>,
    pub taxes: Vec<LineItemsTaxAmount>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodOptionsSofort {
    pub preferred_language: PaymentMethodOptionsSofortPreferredLanguage,
    pub setup_future_usage: Option<PaymentMethodOptionsSofortSetupFutureUsage>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentLinksResourceTransferData {
    pub amount: i64,
    pub destination: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LinkedAccountOptionsUsBankAccount {
    pub permissions: Option<Vec<LinkedAccountOptionsUsBankAccountPermissionsItem>>,
    pub prefetch: Vec<LinkedAccountOptionsUsBankAccountPrefetchItem>,
    pub return_url: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsAffirmSetupFutureUsage {
    None,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxCalculationLineItem {
    pub amount: i64,
    pub amount_tax: i64,
    pub id: String,
    pub livemode: bool,
    pub object: String,
    pub product: String,
    pub quantity: i64,
    pub reference: String,
    pub tax_behavior: TaxCalculationLineItemTaxBehavior,
    pub tax_breakdown: Vec<TaxProductResourceLineItemTaxBreakdown>,
    pub tax_code: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodFpx {
    pub account_holder_type: PaymentMethodFpxAccountHolderType,
    pub bank: PaymentMethodFpxBank,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxProductResourceTaxSettingsDefaults {
    pub tax_behavior: TaxProductResourceTaxSettingsDefaultsTaxBehavior,
    pub tax_code: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupAttemptPaymentMethodDetailsBancontactPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SourceMandateNotification {
    pub acss_debit: Option<SourceMandateNotificationAcssDebitData>,
    pub amount: i64,
    pub bacs_debit: Option<SourceMandateNotificationBacsDebitData>,
    pub created: i64,
    pub id: String,
    pub livemode: bool,
    pub object: String,
    pub reason: String,
    pub sepa_debit: Option<SourceMandateNotificationSepaDebitData>,
    pub source: Source,
    pub status: String,
    pub r#type: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SubscriptionsResourcePendingUpdate {
    pub billing_cycle_anchor: i64,
    pub expires_at: i64,
    pub subscription_items: Vec<SubscriptionItem>,
    pub trial_end: i64,
    pub trial_from_plan: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxProductResourceTaxTransactionShippingCost {
    pub amount: i64,
    pub amount_tax: i64,
    pub shipping_rate: Option<String>,
    pub tax_behavior: TaxProductResourceTaxTransactionShippingCostTaxBehavior,
    pub tax_breakdown: Option<Vec<TaxProductResourceLineItemTaxBreakdown>>,
    pub tax_code: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesLinkPayments {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsWechatPayClient {
    Android,
    Ios,
    Web,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SourceRedirectFlow {
    pub failure_reason: String,
    pub return_url: String,
    pub status: String,
    pub url: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesSofortPayments {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryFinancialAccountsResourceTogglesSettingStatusDetails {
    pub code: TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode,
    pub resolution: TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution,
    pub restriction: Option<
        TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction,
    >,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingNetworkTokenDevice {
    pub device_fingerprint: Option<String>,
    pub ip_address: Option<String>,
    pub location: Option<String>,
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub r#type: Option<IssuingNetworkTokenDeviceType>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CouponDuration {
    Forever,
    Once,
    Repeating,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutKlarnaPaymentMethodOptionsSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BankConnectionsResourceAccountholder {
    pub account: Option<ReplaceMeWithAnyOfSpec>,
    pub customer: Option<ReplaceMeWithAnyOfSpec>,
    pub r#type: BankConnectionsResourceAccountholderType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingDisputeStatus {
    Expired,
    Lost,
    Submitted,
    Unsubmitted,
    Won,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus {
    Available,
    Unavailable,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Coupon {
    pub amount_off: i64,
    pub applies_to: Option<CouponAppliesTo>,
    pub created: i64,
    pub currency: String,
    pub currency_options: Option<CouponCurrencyOptions>,
    pub duration: CouponDuration,
    pub duration_in_months: i64,
    pub id: String,
    pub livemode: bool,
    pub max_redemptions: i64,
    pub metadata: HashMap<String, String>,
    pub name: String,
    pub object: String,
    pub percent_off: f64,
    pub redeem_by: i64,
    pub times_redeemed: i64,
    pub valid: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentNextActionPaynowDisplayQrCode {
    pub data: String,
    pub hosted_instructions_url: String,
    pub image_url_png: String,
    pub image_url_svg: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentLinksResourceShippingOption {
    pub shipping_amount: i64,
    pub shipping_rate: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutP24PaymentMethodOptionsSetupFutureUsage {
    None,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsSofortPreferredLanguage {
    De,
    En,
    Es,
    Fr,
    It,
    Nl,
    Pl,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CustomerBalanceTransactionType {
    Adjustment,
    AppliedToInvoice,
    CreditNote,
    Initial,
    InvoiceOverpaid,
    InvoiceTooLarge,
    InvoiceTooSmall,
    Migration,
    UnappliedFromInvoice,
    UnspentReceiverCredit,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsAchDebit {
    pub account_holder_type: PaymentMethodDetailsAchDebitAccountHolderType,
    pub bank_name: String,
    pub country: String,
    pub fingerprint: String,
    pub last4: String,
    pub routing_number: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentFlowsAmountDetailsResourceTip {
    pub amount: Option<i64>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntentPaymentMethodOptionsCardNetwork {
    Amex,
    CartesBancaires,
    Diners,
    Discover,
    EftposAu,
    Interac,
    Jcb,
    Mastercard,
    Unionpay,
    Unknown,
    Visa,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SourceMandateNotificationBacsDebitData {
    pub last4: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountCapabilitiesBankTransferPayments {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MandateUsBankAccountCollectionMethod {
    Paper,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SubscriptionScheduleCurrentPhase {
    pub end_date: i64,
    pub start_date: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetupIntentPaymentMethodOptionsSepaDebit {
    pub mandate_options: Option<SetupIntentPaymentMethodOptionsMandateOptionsSepaDebit>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodCardPresentNetworks {
    pub available: Vec<String>,
    pub preferred: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsInteracPresentReceiptAccountType {
    Checking,
    Savings,
    Unknown,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TaxProductRegistrationsResourceCountryOptionsUnitedStates {
    pub local_amusement_tax: Option<
        TaxProductRegistrationsResourceCountryOptionsUsLocalAmusementTax,
    >,
    pub local_lease_tax: Option<
        TaxProductRegistrationsResourceCountryOptionsUsLocalLeaseTax,
    >,
    pub state: String,
    pub r#type: TaxProductRegistrationsResourceCountryOptionsUnitedStatesType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetupIntentNextActionRedirectToUrl {
    pub return_url: String,
    pub url: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TerminalLocation {
    pub address: Address,
    pub configuration_overrides: Option<String>,
    pub display_name: String,
    pub id: String,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub object: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalReaderReaderResourceReaderActionType {
    ProcessPaymentIntent,
    ProcessSetupIntent,
    RefundPayment,
    SetReaderDisplay,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TopupStatus {
    Canceled,
    Failed,
    Pending,
    Reversed,
    Succeeded,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentIntentNextActionVerifyWithMicrodeposits {
    pub arrival_date: i64,
    pub hosted_verification_url: String,
    pub microdeposit_type: PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodBoleto {
    pub tax_id: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssuingCardholderIdDocument {
    pub back: ReplaceMeWithAnyOfSpec,
    pub front: ReplaceMeWithAnyOfSpec,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountRestrictedFeaturesItem {
    CardIssuing,
    DepositInsurance,
    FinancialAddressesAba,
    InboundTransfersAch,
    IntraStripeFlows,
    OutboundPaymentsAch,
    OutboundPaymentsUsDomesticWire,
    OutboundTransfersAch,
    OutboundTransfersUsDomesticWire,
    RemoteDepositCapture,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryFinancialAccountsResourceBalanceCash {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoicePaymentMethodOptionsCustomerBalanceFundingType {
    BankTransfer,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsPromptpay {
    pub reference: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Source {
    pub ach_credit_transfer: Option<SourceTypeAchCreditTransfer>,
    pub ach_debit: Option<SourceTypeAchDebit>,
    pub acss_debit: Option<SourceTypeAcssDebit>,
    pub alipay: Option<SourceTypeAlipay>,
    pub amount: i64,
    pub au_becs_debit: Option<SourceTypeAuBecsDebit>,
    pub bancontact: Option<SourceTypeBancontact>,
    pub card: Option<SourceTypeCard>,
    pub card_present: Option<SourceTypeCardPresent>,
    pub client_secret: String,
    pub code_verification: Option<SourceCodeVerificationFlow>,
    pub created: i64,
    pub currency: String,
    pub customer: Option<String>,
    pub eps: Option<SourceTypeEps>,
    pub flow: String,
    pub giropay: Option<SourceTypeGiropay>,
    pub id: String,
    pub ideal: Option<SourceTypeIdeal>,
    pub klarna: Option<SourceTypeKlarna>,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub multibanco: Option<SourceTypeMultibanco>,
    pub object: String,
    pub owner: ReplaceMeWithAnyOfSpec,
    pub p24: Option<SourceTypeP24>,
    pub receiver: Option<SourceReceiverFlow>,
    pub redirect: Option<SourceRedirectFlow>,
    pub sepa_credit_transfer: Option<SourceTypeSepaCreditTransfer>,
    pub sepa_debit: Option<SourceTypeSepaDebit>,
    pub sofort: Option<SourceTypeSofort>,
    pub source_order: Option<SourceOrder>,
    pub statement_descriptor: String,
    pub status: String,
    pub three_d_secure: Option<SourceTypeThreeDSecure>,
    pub r#type: SourceType,
    pub usage: String,
    pub wechat: Option<SourceTypeWechat>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GelatoIdNumberReportError {
    pub code: GelatoIdNumberReportErrorCode,
    pub reason: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountsResourceClosedStatusDetailsReasonsItem {
    AccountRejected,
    ClosedByPlatform,
    Other,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TreasuryFinancialAccountsResourceFinancialAddressesFeatures {
    pub aba: Option<TreasuryFinancialAccountsResourceAbaToggleSettings>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction {
    InboundFlows,
    OutboundFlows,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ExternalAccountRequirements {
    pub currently_due: Vec<String>,
    pub errors: Vec<AccountRequirementsError>,
    pub past_due: Vec<String>,
    pub pending_verification: Vec<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Networks {
    pub available: Vec<String>,
    pub preferred: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GelatoSessionDocumentOptionsAllowedTypesItem {
    DrivingLicense,
    IdCard,
    Passport,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetailsPix {
    pub bank_transaction_id: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MandateUsBankAccount {
    pub collection_method: Option<MandateUsBankAccountCollectionMethod>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentConfirmationMethod {
    Automatic,
    Manual,
}

