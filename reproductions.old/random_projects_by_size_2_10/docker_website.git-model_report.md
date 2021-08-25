# Model report for file:///tmp/top-repos-quality-repos-8losb434/docker_website.git HEAD ecbabb3a1a20a0e6c4346dc16fda1762ce42afc0

### Dump

```json
{'created_at': '2021-08-22 04:38:00',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-81-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '14.9 kB',
 'tags': [],
 'uuid': '6d2e69a7-6ca9-4135-a023-6c0cc5f9f78f',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-8losb434/docker_website.git ecbabb3a1a20a0e6c4346dc16fda1762ce42afc0

# javascript
8 rules, avg.len. 6.0
## train
PPCR: 0.661955
### report
macro
{'f1-score': 0.7745901103349141,
 'precision': 0.8152773195145587,
 'recall': 0.7441110189452931,
 'support': 4543}
micro
{'f1-score': 0.9711644287915474,
 'precision': 0.9711644287915474,
 'recall': 0.9711644287915474,
 'support': 4543}
weighted
{'f1-score': 0.9671373817138716,
 'precision': 0.9657673709861051,
 'recall': 0.9711644287915474,
 'support': 4543}
### report_full
macro
{'f1-score': 0.558827038492855,
 'precision': 0.8152773195145587,
 'recall': 0.48000984963961363,
 'support': 6863}
micro
{'f1-score': 0.7736279151323865,
 'precision': 0.9711644287915474,
 'recall': 0.6428675506338336,
 'support': 6863}
weighted
{'f1-score': 0.7059051725813815,
 'precision': 0.9440822071768797,
 'recall': 0.6428675506338336,
 'support': 6863}
## test
PPCR: 0.741935
### report
macro
{'f1-score': 0.7467637540453075,
 'precision': 0.826984126984127,
 'recall': 0.7023809523809524,
 'support': 621}
micro
{'f1-score': 0.9677938808373591,
 'precision': 0.9677938808373591,
 'recall': 0.9677938808373591,
 'support': 621}
weighted
{'f1-score': 0.963812412384243,
 'precision': 0.9690207806149836,
 'recall': 0.9677938808373591,
 'support': 621}
### report_full
macro
{'f1-score': 0.587210567412096,
 'precision': 0.826984126984127,
 'recall': 0.5323233824983927,
 'support': 837}
micro
{'f1-score': 0.8244170096021948,
 'precision': 0.9677938808373591,
 'recall': 0.7180406212664278,
 'support': 837}
weighted
{'f1-score': 0.7671666620954279,
 'precision': 0.9600386869204074,
 'recall': 0.7180406212664278,
 'support': 837}
```

## javascript
### Summary
8 rules, avg.len. 6.0

| | |
|-|-|
|Min support|93|
|Max support|1731|
|Min confidence|0.9480000138282776|
|Max confidence|0.9956521987915039|

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
                     'min_samples_leaf': 90,
                     'min_samples_split': 240,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.roles in {IDENTIFIER} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 1731.` |
| 2 | `  -1.internal_type = CommentLine<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.973. Support: 200.` |
| 3 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.954. Support: 140.` |
| 4 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = {<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.996. Support: 115.` |
| 5 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {;, {}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = '<br>Confidence: 0.995. Support: 93.` |
| 6 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.948. Support: 125.` |
| 7 | `  -1.diff_col ≥ 4<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 214.` |
| 8 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.reserved not in {(, {}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 1010.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.0, "max_conf": 0.9956521987915039, "max_support": 1731, "min_conf": 0.9480000138282776, "min_support": 93, "num_rules": 8}}
```
</details>
