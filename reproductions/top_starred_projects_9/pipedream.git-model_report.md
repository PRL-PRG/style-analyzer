# Model report for file:///tmp/top-repos-quality-repos-a0qleu_g/pipedream.git HEAD 2ad7b3c7c44cb0262e31151a182044fe578026fc

### Dump

```json
{'created_at': '2021-08-29 12:14:13',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.15.0-135-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '19.1 kB',
 'tags': [],
 'uuid': '401537da-430a-4d23-ab74-fae449d3272c',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-a0qleu_g/pipedream.git 2ad7b3c7c44cb0262e31151a182044fe578026fc

# javascript
55 rules, avg.len. 8.8
## train
PPCR: 0.962573
### report
macro
{'f1-score': 0.7054798563916693,
 'precision': 0.7555736047226685,
 'recall': 0.6823006771711502,
 'support': 140296}
micro
{'f1-score': 0.9695287107258939,
 'precision': 0.9695287107258939,
 'recall': 0.9695287107258939,
 'support': 140296}
weighted
{'f1-score': 0.9656216236256457,
 'precision': 0.9641699320218294,
 'recall': 0.9695287107258939,
 'support': 140296}
### report_full
macro
{'f1-score': 0.6776019890253422,
 'precision': 0.7555736047226685,
 'recall': 0.640321557866963,
 'support': 145751}
micro
{'f1-score': 0.9510395144853818,
 'precision': 0.9695287107258939,
 'recall': 0.9332423105158798,
 'support': 145751}
weighted
{'f1-score': 0.9457108929999513,
 'precision': 0.9629879314907267,
 'recall': 0.9332423105158798,
 'support': 145751}
## test
PPCR: 0.967702
### report
macro
{'f1-score': 0.7143283133736625,
 'precision': 0.7593245311613325,
 'recall': 0.6915408760548085,
 'support': 37152}
micro
{'f1-score': 0.9757482773471146,
 'precision': 0.9757482773471146,
 'recall': 0.9757482773471146,
 'support': 37152}
weighted
{'f1-score': 0.9735755382158566,
 'precision': 0.9733366618785846,
 'recall': 0.9757482773471146,
 'support': 37152}
### report_full
macro
{'f1-score': 0.6900813963176828,
 'precision': 0.7593245311613325,
 'recall': 0.6542582507670515,
 'support': 38392}
micro
{'f1-score': 0.9597320766705496,
 'precision': 0.9757482773471146,
 'recall': 0.9442331735778288,
 'support': 38392}
weighted
{'f1-score': 0.9566369087879059,
 'precision': 0.9728231710233365,
 'recall': 0.9442331735778288,
 'support': 38392}
```

## javascript
### Summary
39 rules, avg.len. 8.0

| | |
|-|-|
|Min support|92|
|Max support|27939|
|Min confidence|0.9205933809280396|
|Max confidence|0.9991150498390198|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 91,
                     'min_samples_split': 182,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.reserved = ,<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.994. Support: 2880.` |
| 2 | `  -1.reserved = ,<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.998. Support: 207.` |
| 3 | `  -1.reserved = ,<br>	∧ +1.reserved not in {], }}<br>	∧ +2.reserved = ]<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 119.` |
| 4 | `  -1.reserved = ,<br>	∧ -5.label not in {<+space>}<br>	∧ +1.reserved not in {], }}<br>	∧ +4.reserved = ,<br>	∧ ^1.roles in {LITERAL}<br>	∧ ^2.roles in {LIST}<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 306.` |
| 5 | `  -1.reserved = ,<br>	∧ -5.label not in {<+space>}<br>	∧ +1.reserved not in {], }}<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.976. Support: 6534.` |
| 6 | `  -1.reserved = ,<br>	∧ -2.roles in {ARGUMENT}<br>	∧ -4.diff_line = 0<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 677.` |
| 7 | `  -1.reserved = ,<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -4.diff_line = 0<br>	∧ -5.reserved = (<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 273.` |
| 8 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {,}<br>⇒ y = "<br>Confidence: 0.933. Support: 5772.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.999. Support: 2825.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 512.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ +1.reserved = }<br>⇒ y = ␣<br>Confidence: 0.921. Support: 573.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -3.reserved = const<br>	∧ +1.reserved not in {=, }}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 158.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -3.reserved not in {const}<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 27939.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_col ≤ 7<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {const}<br>	∧ +2.reserved not in {}}<br>	∧ +3.reserved not in {[}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.983. Support: 4454.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 62<br>⇒ y = ␣<br>Confidence: 0.995. Support: 6781.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, [, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.977. Support: 155.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≤ 12<br>	∧ -2.label not in {<space>}<br>	∧ -3.internal_type = Identifier<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.965. Support: 3621.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≤ 12<br>	∧ -2.label not in {<space>}<br>	∧ -3.internal_type not in {Identifier}<br>	∧ -4.length ≤ 10<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 4<br>	∧ ^2.roles in {RETURN}<br>⇒ y = "<br>Confidence: 0.998. Support: 203.` |
| 19 | `  -1.diff_col ≥ 7<br>	∧ -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, :, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 3<br>⇒ y = ⏎<br>Confidence: 0.928. Support: 606.` |
| 20 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 3<br>	∧ -5.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +5.roles in {MAP}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 187.` |
| 21 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +5.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 5253.` |
| 22 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 3<br>	∧ -3.reserved not in {{}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +5.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.943. Support: 621.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.972. Support: 1650.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = module<br>⇒ y = ⏎⏎<br>Confidence: 0.998. Support: 332.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = =<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 221.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = =<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 3338.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>⇒ y = ∅<br>Confidence: 0.942. Support: 437.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, =, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>⇒ y = ␣<br>Confidence: 0.976. Support: 2058.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, =, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.995. Support: 746.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, =, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.942. Support: 493.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 433.` |
| 32 | `  •••start_col ≤ 33<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.981. Support: 283.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ><br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>⇒ y = ␣<br>Confidence: 0.940. Support: 158.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -5.reserved = const<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 143.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, =, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.roles in {COMMENT}<br>	∧ -5.reserved not in {const}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.986. Support: 106.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = [<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -5.reserved not in {const}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +4.reserved = ,<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.944. Support: 171.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, ;, =, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.internal_type = Identifier<br>	∧ -2.roles not in {COMMENT}<br>	∧ -5.reserved not in {const}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ ^1.roles in {IF} and not in {BODY, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 92.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, =, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -3.reserved not in {;}<br>	∧ -5.reserved not in {const}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {STATEMENT} and not in {IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 2659.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, =, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -5.reserved not in {const}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ ^1.roles not in {IF, OPERATOR, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 19895.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.0, "max_conf": 0.9991150498390198, "max_support": 27939, "min_conf": 0.9205933809280396, "min_support": 92, "num_rules": 39}}
```
</details>
