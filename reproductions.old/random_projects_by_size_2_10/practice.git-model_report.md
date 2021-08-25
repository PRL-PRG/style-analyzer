# Model report for file:///tmp/top-repos-quality-repos-3zqha9m_/practice.git HEAD 477e21eb8fc6b6b77c74507f1b5a1034e67c50de

### Dump

```json
{'created_at': '2021-08-22 03:15:55',
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
 'size': '21.7 kB',
 'tags': [],
 'uuid': 'e87bdb30-1f17-4a5b-9b50-8a3846786521',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-3zqha9m_/practice.git 477e21eb8fc6b6b77c74507f1b5a1034e67c50de

# javascript
68 rules, avg.len. 7.2
## train
PPCR: 0.809016
### report
macro
{'f1-score': 0.27685053533244225,
 'precision': 0.2814304932768304,
 'recall': 0.277659962991332,
 'support': 15631}
micro
{'f1-score': 0.8728168383340797,
 'precision': 0.8728168383340797,
 'recall': 0.8728168383340797,
 'support': 15631}
weighted
{'f1-score': 0.84530519519881,
 'precision': 0.8227316544065055,
 'recall': 0.8728168383340797,
 'support': 15631}
### report_full
macro
{'f1-score': 0.22969025977229396,
 'precision': 0.2814304932768304,
 'recall': 0.20869898772560927,
 'support': 19321}
micro
{'f1-score': 0.7806706340123598,
 'precision': 0.8728168383340797,
 'recall': 0.706122871486983,
 'support': 19321}
weighted
{'f1-score': 0.7175927379970031,
 'precision': 0.752553753307383,
 'recall': 0.706122871486983,
 'support': 19321}
## test
PPCR: 0.827969
### report
macro
{'f1-score': 0.26907081370834635,
 'precision': 0.2739494956297724,
 'recall': 0.27038402393772115,
 'support': 3807}
micro
{'f1-score': 0.8783819280273181,
 'precision': 0.8783819280273181,
 'recall': 0.8783819280273181,
 'support': 3807}
weighted
{'f1-score': 0.8510513334858472,
 'precision': 0.8277784247491816,
 'recall': 0.8783819280273181,
 'support': 3807}
### report_full
macro
{'f1-score': 0.22781223462416014,
 'precision': 0.2739494956297724,
 'recall': 0.20890312885362664,
 'support': 4598}
micro
{'f1-score': 0.7957168352171327,
 'precision': 0.8783819280273181,
 'recall': 0.7272727272727273,
 'support': 4598}
weighted
{'f1-score': 0.7320673740646463,
 'precision': 0.75404204411469,
 'recall': 0.7272727272727273,
 'support': 4598}
```

## javascript
### Summary
28 rules, avg.len. 7.9

| | |
|-|-|
|Min support|129|
|Max support|3428|
|Min confidence|0.9282094836235046|
|Max confidence|0.9991243481636047|

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
                     'base_model_name': 'sklearn.ensemble.RandomForestClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 90,
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.945. Support: 2131.` |
| 2 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 436.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {DECLARATION, FUNCTION} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 205.` |
| 4 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 2131.` |
| 5 | `  -1.reserved = (<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 435.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {DECLARATION, FUNCTION} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 183.` |
| 7 | `  -1.reserved = (<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 444.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = {<br>	∧ ^1.roles in {DECLARATION, FUNCTION} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 209.` |
| 9 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.944. Support: 2144.` |
| 10 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 428.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {DECLARATION, FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.967. Support: 195.` |
| 12 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -3.reserved not in {;}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 9<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {JSXOpeningElement, MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 428.` |
| 13 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -3.reserved not in {;}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 8<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {JSXOpeningElement, MemberExpression}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 173.` |
| 14 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -3.reserved not in {;}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {JSXOpeningElement, MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.937. Support: 3427.` |
| 15 | `  -1.reserved = (<br>	∧ +1.roles in {EXPRESSION} and not in {LITERAL}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 406.` |
| 16 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {INCOMPLETE} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.936. Support: 180.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {DECLARATION} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 182.` |
| 18 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -3.reserved not in {;}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 9<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 450.` |
| 19 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -3.reserved not in {;}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 8<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 129.` |
| 20 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -3.reserved not in {;}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 3428.` |
| 21 | `  -1.reserved = (<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 466.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {{}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {DECLARATION, FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 227.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {{}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {DECLARATION} and not in {FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.928. Support: 592.` |
| 24 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 9<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {JSXOpeningElement, MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 358.` |
| 25 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 8<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {JSXOpeningElement, MemberExpression}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 158.` |
| 26 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label not in {<newline>}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {JSXOpeningElement, MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 571.` |
| 27 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.label not in {<newline>}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {JSXOpeningElement, MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 2909.` |
| 28 | `  -1.reserved = (<br>	∧ +1.roles in {EXPRESSION} and not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 383.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.857142857142857, "max_conf": 0.9991243481636047, "max_support": 3428, "min_conf": 0.9282094836235046, "min_support": 129, "num_rules": 28}}
```
</details>
